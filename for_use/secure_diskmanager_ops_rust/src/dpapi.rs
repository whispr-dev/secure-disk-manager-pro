use crate::error::{Result, SdmError};
use std::fs;
use std::path::Path;

fn hex_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len() * 2);
    for byte in data {
        out.push_str(&format!("{byte:02X}"));
    }
    out
}

fn hex_decode(hex: &str) -> Result<Vec<u8>> {
    let clean: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.len() % 2 != 0 {
        return Err(SdmError::InvalidFormat("hex string has odd length".to_string()));
    }

    let mut out = Vec::with_capacity(clean.len() / 2);
    for pair in clean.as_bytes().chunks(2) {
        let s = std::str::from_utf8(pair).map_err(|e| SdmError::InvalidFormat(e.to_string()))?;
        out.push(u8::from_str_radix(s, 16).map_err(|e| SdmError::InvalidFormat(e.to_string()))?);
    }
    Ok(out)
}

fn extract_ss(xml: &str) -> Result<String> {
    let start_tag = "<SS>";
    let end_tag = "</SS>";
    let start = xml
        .find(start_tag)
        .ok_or_else(|| SdmError::InvalidFormat("missing <SS>".to_string()))?
        + start_tag.len();
    let end = xml[start..]
        .find(end_tag)
        .ok_or_else(|| SdmError::InvalidFormat("missing </SS>".to_string()))?
        + start;
    Ok(xml[start..end].trim().to_string())
}

/// C++ equivalent: `DPAPIEncryptor::encrypt_to_xml`.
pub fn encrypt_to_xml(plaintext: &str, out_path: impl AsRef<Path>) -> Result<()> {
    let protected = dpapi_protect(plaintext.as_bytes())?;
    let xml = format!("<Objs><SS>{}</SS></Objs>\n", hex_encode(&protected));
    fs::write(out_path, xml)?;
    Ok(())
}

/// C++ equivalent: `DPAPIDecryptor::decrypt_from_xml`.
pub fn decrypt_from_xml(xml_path: impl AsRef<Path>) -> Result<String> {
    let xml = fs::read_to_string(xml_path)?;
    let hex = extract_ss(&xml)?;
    let protected = hex_decode(&hex)?;
    let plain = dpapi_unprotect(&protected)?;
    String::from_utf8(plain).map_err(|e| SdmError::Crypto(e.to_string()))
}

#[cfg(windows)]
#[allow(non_snake_case)]
mod win_dpapi {
    use super::{Result, SdmError};
    use std::ffi::c_void;
    use std::ptr::{null, null_mut};

    /// Windows `DATA_BLOB` / `CRYPT_INTEGER_BLOB` ABI layout.
    ///
    /// This is defined locally instead of imported from `windows-sys` because
    /// crate versions expose the alias under different names. The ABI is stable:
    /// `{ DWORD cbData; BYTE *pbData; }`.
    #[repr(C)]
    struct DataBlob {
        cb_data: u32,
        pb_data: *mut u8,
    }

    #[link(name = "crypt32")]
    extern "system" {
        fn CryptProtectData(
            data_in: *mut DataBlob,
            data_descr: *const u16,
            optional_entropy: *mut DataBlob,
            reserved: *mut c_void,
            prompt_struct: *mut c_void,
            flags: u32,
            data_out: *mut DataBlob,
        ) -> i32;

        fn CryptUnprotectData(
            data_in: *mut DataBlob,
            data_descr: *mut *mut u16,
            optional_entropy: *mut DataBlob,
            reserved: *mut c_void,
            prompt_struct: *mut c_void,
            flags: u32,
            data_out: *mut DataBlob,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn LocalFree(mem: *mut c_void) -> *mut c_void;
    }

    fn blob_to_vec(blob: &DataBlob) -> Result<Vec<u8>> {
        if blob.cb_data == 0 {
            return Ok(Vec::new());
        }
        if blob.pb_data.is_null() {
            return Err(SdmError::Crypto("DPAPI returned a null data pointer".to_string()));
        }
        Ok(unsafe { std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize).to_vec() })
    }

    pub(super) fn protect(data: &[u8]) -> Result<Vec<u8>> {
        let mut input = DataBlob {
            cb_data: data.len() as u32,
            pb_data: data.as_ptr() as *mut u8,
        };
        let mut output = DataBlob {
            cb_data: 0,
            pb_data: null_mut(),
        };

        let ok = unsafe {
            CryptProtectData(
                &mut input,
                null(),
                null_mut(),
                null_mut(),
                null_mut(),
                0,
                &mut output,
            )
        };
        if ok == 0 {
            return Err(SdmError::Crypto("CryptProtectData failed".to_string()));
        }

        let protected = blob_to_vec(&output);
        if !output.pb_data.is_null() {
            unsafe {
                let _ = LocalFree(output.pb_data.cast::<c_void>());
            }
        }
        protected
    }

    pub(super) fn unprotect(data: &[u8]) -> Result<Vec<u8>> {
        let mut input = DataBlob {
            cb_data: data.len() as u32,
            pb_data: data.as_ptr() as *mut u8,
        };
        let mut output = DataBlob {
            cb_data: 0,
            pb_data: null_mut(),
        };

        let ok = unsafe {
            CryptUnprotectData(
                &mut input,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                0,
                &mut output,
            )
        };
        if ok == 0 {
            return Err(SdmError::Crypto("CryptUnprotectData failed".to_string()));
        }

        let plain = blob_to_vec(&output);
        if !output.pb_data.is_null() {
            unsafe {
                let _ = LocalFree(output.pb_data.cast::<c_void>());
            }
        }
        plain
    }
}

#[cfg(windows)]
fn dpapi_protect(data: &[u8]) -> Result<Vec<u8>> {
    win_dpapi::protect(data)
}

#[cfg(windows)]
fn dpapi_unprotect(data: &[u8]) -> Result<Vec<u8>> {
    win_dpapi::unprotect(data)
}

#[cfg(not(windows))]
fn dpapi_protect(_data: &[u8]) -> Result<Vec<u8>> {
    Err(SdmError::UnsupportedPlatform("Windows DPAPI is only available on Windows"))
}

#[cfg(not(windows))]
fn dpapi_unprotect(_data: &[u8]) -> Result<Vec<u8>> {
    Err(SdmError::UnsupportedPlatform("Windows DPAPI is only available on Windows"))
}

/// C++ equivalent: `GhostScriptVault::decrypt_script`.
pub fn decrypt_script(xml_path: impl AsRef<Path>) -> Result<String> {
    decrypt_from_xml(xml_path)
}

/// C++ equivalent name: `GhostScriptVault::execute_decrypted_script`.
///
/// Deliberately blocked: hidden PowerShell execution from decrypted blobs was
/// not ported.
pub fn execute_decrypted_script(_script_contents: &str) -> Result<()> {
    Err(SdmError::Blocked("decrypted script execution was not ported"))
}
