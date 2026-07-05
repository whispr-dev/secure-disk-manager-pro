use crate::error::{Result, SdmError};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::cmp::min;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn zlib_compress(data: &[u8], level: Compression) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), level);
    encoder.write_all(data)?;
    encoder.finish().map_err(SdmError::Io)
}

fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}

fn xor_crypt(data: &mut [u8], key: &[u8]) {
    if key.is_empty() {
        return;
    }
    for (idx, byte) in data.iter_mut().enumerate() {
        *byte ^= key[idx % key.len()];
    }
}

fn is_low_entropy_legacy(data: &[u8]) -> bool {
    if data.len() < 64 {
        return false;
    }
    let first = &data[..32];
    data[32..]
        .chunks_exact(32)
        .all(|chunk| chunk == first)
}

fn read_u32_le(buf: &[u8], pos: &mut usize) -> Result<u32> {
    if *pos + 4 > buf.len() {
        return Err(SdmError::InvalidFormat("unexpected EOF while reading u32".to_string()));
    }
    let value = u32::from_le_bytes(buf[*pos..*pos + 4].try_into().expect("fixed length"));
    *pos += 4;
    Ok(value)
}

fn read_u64_le(buf: &[u8], pos: &mut usize) -> Result<u64> {
    if *pos + 8 > buf.len() {
        return Err(SdmError::InvalidFormat("unexpected EOF while reading u64".to_string()));
    }
    let value = u64::from_le_bytes(buf[*pos..*pos + 8].try_into().expect("fixed length"));
    *pos += 8;
    Ok(value)
}

/// C++ equivalent: `FileCompressor::compress`.
///
/// Format-compatible with the Windows/MSVC build: `[u32 original_len_le][zlib payload]`.
pub fn compress(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    let input = fs::read(input_path)?;
    if input.len() > u32::MAX as usize {
        return Err(SdmError::InvalidInput(
            "legacy FileCompressor format is limited to 4 GiB".to_string(),
        ));
    }
    let compressed = zlib_compress(&input, Compression::fast())?;
    let mut out = Vec::with_capacity(4 + compressed.len());
    out.extend_from_slice(&(input.len() as u32).to_le_bytes());
    out.extend_from_slice(&compressed);
    fs::write(output_path, out)?;
    Ok(())
}

/// C++ equivalent: `FileCompressor::decompress`.
pub fn decompress(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    let input = fs::read(input_path)?;
    let mut pos = 0;
    let expected_len = read_u32_le(&input, &mut pos)? as usize;
    let output = zlib_decompress(&input[pos..])?;
    if output.len() != expected_len {
        return Err(SdmError::Compression(format!(
            "decompressed length mismatch: expected {expected_len}, got {}",
            output.len()
        )));
    }
    fs::write(output_path, output)?;
    Ok(())
}

/// C++ equivalent: `FileCompressorMT::compress`.
///
/// Legacy chunk format:
/// `[u32 chunks][u32 chunk_size][u64 original_size]` then repeated
/// `[u32 stored_size][stored_bytes]`, where `stored_size == 0` means raw chunk.
pub fn compress_mt(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>, chunk_size: usize) -> Result<()> {
    if chunk_size == 0 || chunk_size > u32::MAX as usize {
        return Err(SdmError::InvalidInput("chunk_size must be 1..=u32::MAX".to_string()));
    }
    let input = fs::read(input_path)?;
    let original_size = input.len() as u64;
    let num_chunks = if input.is_empty() { 0 } else { (input.len() + chunk_size - 1) / chunk_size };
    if num_chunks > u32::MAX as usize {
        return Err(SdmError::InvalidInput("too many chunks for legacy format".to_string()));
    }

    let mut out = Vec::new();
    out.extend_from_slice(&(num_chunks as u32).to_le_bytes());
    out.extend_from_slice(&(chunk_size as u32).to_le_bytes());
    out.extend_from_slice(&original_size.to_le_bytes());

    for chunk in input.chunks(chunk_size) {
        if is_low_entropy_legacy(chunk) {
            out.extend_from_slice(&0_u32.to_le_bytes());
            out.extend_from_slice(chunk);
        } else {
            let compressed = zlib_compress(chunk, Compression::fast())?;
            if compressed.len() > u32::MAX as usize {
                return Err(SdmError::Compression("compressed chunk too large".to_string()));
            }
            out.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
            out.extend_from_slice(&compressed);
        }
    }

    fs::write(output_path, out)?;
    Ok(())
}

/// C++ equivalent: `FileCompressorMT::decompress`.
pub fn decompress_mt(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    let input = fs::read(input_path)?;
    let mut pos = 0;
    let num_chunks = read_u32_le(&input, &mut pos)? as usize;
    let chunk_size = read_u32_le(&input, &mut pos)? as usize;
    let original_size = read_u64_le(&input, &mut pos)? as usize;

    let mut output = Vec::with_capacity(original_size);
    for chunk_idx in 0..num_chunks {
        let stored_size = read_u32_le(&input, &mut pos)? as usize;
        let remaining = original_size.saturating_sub(output.len());
        let out_size = min(chunk_size, remaining);
        if stored_size == 0 {
            if pos + out_size > input.len() {
                return Err(SdmError::InvalidFormat("unexpected EOF in raw chunk".to_string()));
            }
            output.extend_from_slice(&input[pos..pos + out_size]);
            pos += out_size;
        } else {
            if pos + stored_size > input.len() {
                return Err(SdmError::InvalidFormat("unexpected EOF in compressed chunk".to_string()));
            }
            let decoded = zlib_decompress(&input[pos..pos + stored_size])?;
            pos += stored_size;
            if decoded.len() != out_size && chunk_idx + 1 != num_chunks {
                return Err(SdmError::Compression("chunk length mismatch".to_string()));
            }
            output.extend_from_slice(&decoded);
        }
    }

    output.truncate(original_size);
    fs::write(output_path, output)?;
    Ok(())
}

/// C++ equivalent: `FileCompressorMTEnc::compress`.
///
/// This preserves the legacy XOR-after-compression behavior. It is compatibility
/// obfuscation, not modern encryption.
pub fn compress_mt_enc<P: AsRef<Path>, Q: AsRef<Path>, K: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    keyfile: Option<K>,
    mode: &str,
) -> Result<()> {
    let input = fs::read(input_path)?;
    let key = match keyfile {
        Some(path) => fs::read(path)?,
        None => Vec::new(),
    };
    let chunk_size = if mode == "ratio" { 512 * 1024 } else { 2 * 1024 * 1024 };
    let level = if mode == "ratio" { Compression::best() } else { Compression::fast() };
    let original_size = input.len() as u64;
    let num_chunks = if input.is_empty() { 0 } else { (input.len() + chunk_size - 1) / chunk_size };

    let mut out = Vec::new();
    out.extend_from_slice(&(num_chunks as u32).to_le_bytes());
    out.extend_from_slice(&(chunk_size as u32).to_le_bytes());
    out.extend_from_slice(&original_size.to_le_bytes());

    for chunk in input.chunks(chunk_size) {
        let mut stored = if is_low_entropy_legacy(chunk) {
            chunk.to_vec()
        } else {
            zlib_compress(chunk, level)?
        };
        let stored_size = if is_low_entropy_legacy(chunk) { 0_u32 } else { stored.len() as u32 };
        xor_crypt(&mut stored, &key);
        out.extend_from_slice(&stored_size.to_le_bytes());
        out.extend_from_slice(&stored);
    }

    fs::write(output_path, out)?;
    Ok(())
}

/// C++ equivalent: `FileCompressorMTEnc::decompress`.
pub fn decompress_mt_enc<P: AsRef<Path>, Q: AsRef<Path>, K: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    keyfile: Option<K>,
) -> Result<()> {
    let input = fs::read(input_path)?;
    let key = match keyfile {
        Some(path) => fs::read(path)?,
        None => Vec::new(),
    };
    let mut pos = 0;
    let num_chunks = read_u32_le(&input, &mut pos)? as usize;
    let chunk_size = read_u32_le(&input, &mut pos)? as usize;
    let original_size = read_u64_le(&input, &mut pos)? as usize;
    let mut output = Vec::with_capacity(original_size);

    for _ in 0..num_chunks {
        let stored_size = read_u32_le(&input, &mut pos)? as usize;
        let remaining = original_size.saturating_sub(output.len());
        let out_size = min(chunk_size, remaining);
        let take = if stored_size == 0 { out_size } else { stored_size };
        if pos + take > input.len() {
            return Err(SdmError::InvalidFormat("unexpected EOF in encrypted chunk".to_string()));
        }
        let mut stored = input[pos..pos + take].to_vec();
        pos += take;
        xor_crypt(&mut stored, &key);
        if stored_size == 0 {
            output.extend_from_slice(&stored);
        } else {
            output.extend_from_slice(&zlib_decompress(&stored)?);
        }
    }

    output.truncate(original_size);
    fs::write(output_path, output)?;
    Ok(())
}
