# Secure DiskManager Ops Rust v0.2.0 — Full User Manual

Manual version: `2026-07-05`  
Crate version covered: `secure_diskmanager_ops` `v0.2.0`  
Primary binary name: `secure_diskmanager_ops.exe` on Windows, `secure_diskmanager_ops` on Linux/macOS.

---

## 1. What this project is

`secure_diskmanager_ops` is a Rust crate and command-line tool created from two source batches:

1. The original `Secure_DiskManager_Pro` C++ archive.
2. The later `python_library_demos_sm0l.tar.gz` Python demo archive.

The crate has two intended uses:

1. **Runnable CLI utility** — use `secure_diskmanager_ops.exe` from PowerShell / terminal.
2. **Rust library crate** — import modules such as `compression`, `search`, `rng`, `python_ops`, etc. into another Rust program.

It is intentionally **not** a stealth, malware, covert-exfiltration, or wipe automation framework. The original archive contained some modules with names and behaviours around stealth dispatch, MAC spoofing, Tor payload sending, kill-switch wiping, hidden script execution, persistence, and self-delete. Those are represented as explicit blocked functions returning `SdmError::Blocked(...)`.

The useful safe/local operators were ported: compression, entropy, file search, keyfile generation, DPAPI XML protect/unprotect on Windows, RC4 legacy compatibility, local secure deletion of a single regular file, GPG wrapper calls, simple identity/persona planning, quantum/Hilbert helpers, and the Python-demo equivalents.

---

## 2. Project layout

Expected folder structure after extracting the archive:

```text
secure_diskmanager_ops_rust/
  Cargo.toml
  README.md
  BUILD_WINDOWS.md
  FUNCTION_MAP.md
  PYTHON_DEMO_FUNCTION_MAP.md
  build_windows.ps1
  examples/
    demo.rs
    python_ops_demo.rs
  src/
    main.rs
    lib.rs
    cli.rs
    compression.rs
    disk_management.rs
    dpapi.rs
    encryption.rs
    entropy.rs
    error.rs
    file_transfer.rs
    fs_monitor.rs
    ghost_controller.rs
    gpg_wrapper.rs
    identity.rs
    key_manager.rs
    net_admin.rs
    payload_dispatcher.rs
    process_services.rs
    python_ops.rs
    quantum.rs
    rng.rs
    search.rs
    secure_deletion.rs
    stealth_mailer.rs
    system_monitor.rs
```

Important files:

| File | Purpose |
|---|---|
| `Cargo.toml` | Rust package definition, dependencies, binary target, examples. |
| `src/lib.rs` | Library module exports. |
| `src/main.rs` | CLI executable wrapper. This is what creates `secure_diskmanager_ops.exe`. |
| `examples/demo.rs` | Smoke-test demo for core C++-derived operators. |
| `examples/python_ops_demo.rs` | Smoke-test demo for Python-demo-derived operators. |
| `FUNCTION_MAP.md` | Mapping from C++ source/header functions to Rust modules. |
| `PYTHON_DEMO_FUNCTION_MAP.md` | Mapping from Python demo scripts to `python_ops.rs`. |
| `build_windows.ps1` | Windows build/check helper. |

---

## 3. Install requirements

### 3.1 Required

You need the Rust toolchain:

```powershell
rustc --version
cargo --version
```

If those commands fail, install Rust from `rustup` first.

### 3.2 Optional external tools

Some library-only modules shell out to host tools:

| Tool | Used by | Required for CLI? |
|---|---|---|
| `gpg` | `gpg_wrapper`, parts of `identity::ProfileRotator` | No, unless your own Rust code calls those APIs. |
| `curl` | `file_transfer::upload/download` | No, not currently exposed by the CLI. |
| `sc` | Windows service control in `process_services` | Library-only. |
| `systemctl` | Linux service control in `process_services` | Library-only. |
| `rasdial` | Windows VPN start/status | Library-only. |
| `nmcli` | Linux VPN start/status | Library-only. |

---

## 4. Building on Windows

Open PowerShell in the crate root:

```powershell
cd C:\github\secure-disk-manager-pro\secure_diskmanager_ops_rust
```

### 4.1 Build the main CLI binary

```powershell
cargo build --bin secure_diskmanager_ops
```

Expected output file:

```text
target\debug\secure_diskmanager_ops.exe
```

### 4.2 Build release binary

```powershell
cargo build --release --bin secure_diskmanager_ops
```

Expected output file:

```text
target\release\secure_diskmanager_ops.exe
```

Use release builds when you want the portable binary to be faster and smaller.

### 4.3 Build demos

```powershell
cargo build --example demo
cargo build --example python_ops_demo
```

Expected output files:

```text
target\debug\examples\demo.exe
target\debug\examples\python_ops_demo.exe
```

### 4.4 Run the build helper

```powershell
.\build_windows.ps1
```

That script is intended to run the main build and example builds, then check for expected output files.

---

## 5. Important Cargo output locations

Cargo creates several different artifact types.

| Artifact | Example path | What it is |
|---|---|---|
| Main CLI executable | `target\debug\secure_diskmanager_ops.exe` | The actual app. |
| Release CLI executable | `target\release\secure_diskmanager_ops.exe` | Optimized app. |
| Demo executable | `target\debug\examples\demo.exe` | Example smoke test only. |
| Python ops demo executable | `target\debug\examples\python_ops_demo.exe` | Example smoke test only. |
| Rust library artifact | `target\debug\deps\libsecure_diskmanager_ops-*.rlib` | Linkable library for other Rust crates, not something you run. |
| Dependency build scripts | `target\debug\build\...\build_script_build.exe` | Cargo dependency machinery. Ignore these as user-facing apps. |

If the only executable you see is under `target\debug\build\...`, that is not your app. It is usually a dependency build script.

To search for every produced `.exe`:

```powershell
Get-ChildItem -Path .\target -Recurse -Filter *.exe | Select-Object -ExpandProperty FullName
```

---

## 6. Running without manually finding the `.exe`

Cargo can build and run the CLI directly:

```powershell
cargo run --bin secure_diskmanager_ops -- help
```

Everything after `--` is passed to the program.

Examples:

```powershell
cargo run --bin secure_diskmanager_ops -- disk-usage .
cargo run --bin secure_diskmanager_ops -- search . "\.rs$" --entropy
cargo run --bin secure_diskmanager_ops -- py-demo-report
```

This is the easiest way to test commands while developing.

---

## 7. CLI overview

General form:

```powershell
.\target\debug\secure_diskmanager_ops.exe <command> [args]
```

or:

```powershell
cargo run --bin secure_diskmanager_ops -- <command> [args]
```

Help:

```powershell
.\target\debug\secure_diskmanager_ops.exe help
```

The CLI is implemented in `src/main.rs`. It exposes a selected safe subset of the library. Many library APIs are intentionally not wired into CLI commands because they are lower-level building blocks or require application-specific context.

---

## 8. CLI command reference

### 8.1 `help`

Show built-in help text.

```powershell
.\target\debug\secure_diskmanager_ops.exe help
```

Aliases:

```powershell
.\target\debug\secure_diskmanager_ops.exe --help
.\target\debug\secure_diskmanager_ops.exe -h
```

---

### 8.2 `disk-usage <path>`

Sums the byte size of regular files immediately inside `<path>`.

```powershell
.\target\debug\secure_diskmanager_ops.exe disk-usage .
```

Important behaviour: this preserves the C++ implementation behaviour and **does not recurse** into subdirectories.

Example output:

```text
123456
```

Meaning: immediate regular files inside the path total 123,456 bytes.

Library function:

```rust
secure_diskmanager_ops::disk_management::get_disk_usage(path)
```

---

### 8.3 `search <root> <regex> [--entropy]`

Walks recursively under `<root>` and matches file paths against a regular expression.

```powershell
.\target\debug\secure_diskmanager_ops.exe search . "\.rs$"
```

With entropy score per hit:

```powershell
.\target\debug\secure_diskmanager_ops.exe search . "\.rs$" --entropy
```

PowerShell quoting note: regex backslashes are easiest inside double quotes:

```powershell
"\.rs$"
```

Output format without entropy:

```text
C:\path\to\file.rs
```

Output format with entropy:

```text
C:\path\to\file.rs | Entropy: 4.123456
```

The command also prints hit count to stderr:

```text
12 hit(s)
```

Library functions:

```rust
search::find_matching_files(root, pattern, use_entropy)
search::format_search_results(&hits)
```

---

### 8.4 `ultra-search <root> <substring> [--csv <out.csv>]`

Performs a recursive substring match against file paths. This is simpler than regex search and returns metadata.

```powershell
.\target\debug\secure_diskmanager_ops.exe ultra-search . Cargo
```

Export to CSV:

```powershell
.\target\debug\secure_diskmanager_ops.exe ultra-search . Cargo --csv results.csv
```

Output format:

```text
C:\path\Cargo.toml | Size: 1234 bytes | ModifiedUnix: 1783235295
```

CSV format:

```csv
Path,Size,ModifiedUnix
"C:\path\Cargo.toml",1234,1783235295
```

Library functions:

```rust
search::ultra_fast_search(pattern, root)
search::format_file_entries(&hits)
search::export_to_csv(&hits, out_file)
```

---

### 8.5 `entropy <file>`

Calculates Shannon entropy for a file.

```powershell
.\target\debug\secure_diskmanager_ops.exe entropy .\Cargo.toml
```

Example output:

```text
4.81234567
```

Interpretation is approximate:

| Entropy range | Rough meaning |
|---:|---|
| `0.0` | Empty or all one repeated byte. |
| `1.0` to `4.0` | Structured text or simple repetitive data. |
| `4.0` to `6.5` | Typical text/source/config/mixed files. |
| `7.0` to `8.0` | Highly random, compressed, or encrypted-looking bytes. |

Library functions:

```rust
entropy::calculate_entropy_bytes(data)
entropy::calculate_entropy(text)
entropy::calculate_file_entropy(path)
```

---

### 8.6 `rng-keyfile <out-file> <num-bytes>`

Writes OS-random bytes to a keyfile.

```powershell
.\target\debug\secure_diskmanager_ops.exe rng-keyfile .\my.key 32
```

Example output:

```text
wrote 32 random byte(s) to .\my.key
```

Useful sizes:

| Bytes | Common interpretation |
|---:|---|
| `16` | 128-bit key material. |
| `24` | 192-bit key material. |
| `32` | 256-bit key material. |
| `64` | Larger seed/key blob. |

Library functions:

```rust
rng::initialize()
rng::get_random_bytes(size)
rng::get_random_64()
rng::get_random_in_range(min, max)
rng::write_keyfile(path, num_bytes)
```

---

### 8.7 `compress <input> <output>`

Compresses a file using the legacy single-file zlib format:

```text
[u32 original_len_le][zlib payload]
```

Command:

```powershell
.\target\debug\secure_diskmanager_ops.exe compress input.bin input.bin.z
```

Output:

```text
compressed input.bin -> input.bin.z
```

Notes:

- This format stores the original length as a 32-bit integer.
- It is therefore limited to files up to 4 GiB.
- It is intended for compatibility with the original C++ format, not as a general archive format.

Library function:

```rust
compression::compress(input_path, output_path)
```

---

### 8.8 `decompress <input> <output>`

Decompresses the legacy single-file zlib format produced by `compress`.

```powershell
.\target\debug\secure_diskmanager_ops.exe decompress input.bin.z restored.bin
```

Output:

```text
decompressed input.bin.z -> restored.bin
```

The decompressor validates that the restored length matches the stored original length.

Library function:

```rust
compression::decompress(input_path, output_path)
```

---

### 8.9 `compress-mt <input> <output> [chunk-size]`

Compresses a file using the legacy chunked compressor format.

```powershell
.\target\debug\secure_diskmanager_ops.exe compress-mt input.bin input.bin.mtz
```

With explicit chunk size:

```powershell
.\target\debug\secure_diskmanager_ops.exe compress-mt input.bin input.bin.mtz 1048576
```

Default chunk size:

```text
1048576 bytes
```

Format:

```text
[u32 chunks][u32 chunk_size][u64 original_size]
then repeated:
[u32 stored_size][stored_bytes]
```

A `stored_size` of `0` means the chunk was stored raw.

Library function:

```rust
compression::compress_mt(input_path, output_path, chunk_size)
```

---

### 8.10 `decompress-mt <input> <output>`

Decompresses the legacy chunked compressor format.

```powershell
.\target\debug\secure_diskmanager_ops.exe decompress-mt input.bin.mtz restored.bin
```

Library function:

```rust
compression::decompress_mt(input_path, output_path)
```

---

### 8.11 `compress-mt-enc <input> <output> <keyfile-or-> [fast|ratio]`

Compresses with the chunked format and applies legacy XOR obfuscation after compression.

```powershell
.\target\debug\secure_diskmanager_ops.exe rng-keyfile my.key 32
.\target\debug\secure_diskmanager_ops.exe compress-mt-enc input.bin input.bin.mte my.key fast
```

Use `-` instead of a keyfile for no XOR key:

```powershell
.\target\debug\secure_diskmanager_ops.exe compress-mt-enc input.bin input.bin.mte - fast
```

Modes:

| Mode | Chunk size | Compression level |
|---|---:|---|
| `fast` | 2 MiB | Fast zlib. |
| `ratio` | 512 KiB | Best zlib. |

Important security note: this is **legacy XOR obfuscation**, not modern encryption. Do not use it as a security boundary.

Library function:

```rust
compression::compress_mt_enc(input_path, output_path, keyfile_option, mode)
```

---

### 8.12 `decompress-mt-enc <input> <output> <keyfile-or->`

Decompresses the chunked XOR-obfuscated legacy format.

```powershell
.\target\debug\secure_diskmanager_ops.exe decompress-mt-enc input.bin.mte restored.bin my.key
```

With no keyfile:

```powershell
.\target\debug\secure_diskmanager_ops.exe decompress-mt-enc input.bin.mte restored.bin -
```

Library function:

```rust
compression::decompress_mt_enc(input_path, output_path, keyfile_option)
```

---

### 8.13 `rc4-encrypt <input> <key-string>`

Applies the legacy RC4 transform to a file and writes `<input>.enc`.

```powershell
.\target\debug\secure_diskmanager_ops.exe rc4-encrypt secret.txt "my passphrase"
```

Output:

```text
wrote secret.txt.enc
```

Important security note: RC4 is obsolete and should not be used for new serious cryptographic protection. This exists for compatibility with the original C++ operator.

Library functions:

```rust
encryption::initialize_sbox(key_bytes)
encryption::rc4_encrypt_decrypt(&mut data, key_bytes)
encryption::encrypt_file(file_path, key_string)
```

---

### 8.14 `rc4-decrypt <input> <key-string>`

Applies the same RC4 transform to decrypt and writes `<input>.dec`.

```powershell
.\target\debug\secure_diskmanager_ops.exe rc4-decrypt secret.txt.enc "my passphrase"
```

Because RC4 is symmetric, encryption and decryption are the same transform. The wrapper names only control output suffixes.

Library function:

```rust
encryption::decrypt_file(file_path, key_string)
```

---

### 8.15 `rc4-encrypt-keyfile <input> <keyfile>`

Encrypts using raw bytes loaded from a keyfile and writes `<input>.enc`.

```powershell
.\target\debug\secure_diskmanager_ops.exe rng-keyfile rc4.key 32
.\target\debug\secure_diskmanager_ops.exe rc4-encrypt-keyfile secret.txt rc4.key
```

Library function:

```rust
encryption::encrypt_file_with_keyfile(file_path, keyfile_path)
```

---

### 8.16 `rc4-decrypt-keyfile <input> <keyfile>`

Decrypts using raw bytes loaded from a keyfile and writes `<input>.dec`.

```powershell
.\target\debug\secure_diskmanager_ops.exe rc4-decrypt-keyfile secret.txt.enc rc4.key
```

Library function:

```rust
encryption::decrypt_file_with_keyfile(file_path, keyfile_path)
```

---

### 8.17 `dpapi-protect <plain-text> <out.xml>`

Windows-only. Protects plaintext using the current Windows user’s DPAPI context and writes a small XML file:

```powershell
.\target\debug\secure_diskmanager_ops.exe dpapi-protect "hello secret" secret.xml
```

Output:

```text
wrote DPAPI XML: secret.xml
```

The file shape is:

```xml
<Objs><SS>HEX_PROTECTED_BYTES</SS></Objs>
```

Important notes:

- DPAPI-protected data is tied to the Windows user/machine context.
- A different user or machine usually cannot unprotect it.
- On non-Windows platforms this returns `UnsupportedPlatform`.

Library function:

```rust
dpapi::encrypt_to_xml(plaintext, out_path)
```

---

### 8.18 `dpapi-unprotect <in.xml>`

Windows-only. Reads a DPAPI XML file and prints the plaintext.

```powershell
.\target\debug\secure_diskmanager_ops.exe dpapi-unprotect secret.xml
```

Library function:

```rust
dpapi::decrypt_from_xml(xml_path)
```

Related blocked function:

```rust
dpapi::execute_decrypted_script(script_contents)
```

That returns `SdmError::Blocked` because hidden execution of decrypted scripts was intentionally not ported.

---

### 8.19 `shred <file> [passes]`

Overwrites and deletes a single regular file.

```powershell
.\target\debug\secure_diskmanager_ops.exe shred scratch.bin
```

With explicit pass count:

```powershell
.\target\debug\secure_diskmanager_ops.exe shred scratch.bin 5
```

Default passes:

```text
3
```

Safety behaviour:

- Refuses symlinks.
- Refuses directories.
- Does not recurse.
- Overwrites the file with random data for each pass, flushes/syncs, then removes it.

Practical limitation: on SSDs, journaling filesystems, cloud-synced folders, snapshots, and wear-levelled media, overwrite-based deletion cannot guarantee physical erasure of every historical copy. Treat it as a best-effort local overwrite-and-remove operator, not absolute forensic destruction.

Library functions:

```rust
secure_deletion::shred_file(path, passes)
secure_deletion::shred_file_simd_pattern(path, passes)
```

`shred_file_simd_pattern` uses random bytes for pass 1, `0xFF` for pass 2, and `0x00` for pass 3+.

---

## 9. Python-demo CLI command reference

These commands come from the v0.2.0 Python demo expansion in `src/python_ops.rs`.

### 9.1 `py-demo-list`

Prints the Python demo filenames represented by Rust equivalents.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-demo-list
```

Library function:

```rust
python_ops::represented_python_demos()
```

---

### 9.2 `py-demo-report`

Runs a compact report exercising many Python-demo equivalent operators.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-demo-report
```

It demonstrates date/time helpers, humanized sizes, nested values, HTML extraction, fake data, fuzzy matching, table formatting, matrix multiplication, identifier detection, and geometry.

Library function:

```rust
python_ops::demo_report()
```

---

### 9.3 `py-html <tag> <html>`

Extracts text from simple HTML tags.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-html h1 "<html><body><h1>Hello</h1></body></html>"
```

Example output:

```text
Hello
```

Notes:

- This is not a full browser-grade HTML parser.
- It is a small std-only helper matching the original BeautifulSoup-style demo intent.

Library functions:

```rust
python_ops::extract_first_tag_text(html, tag)
python_ops::extract_tag_texts(html, tag)
python_ops::strip_tags(fragment)
python_ops::html_unescape_basic(text)
```

---

### 9.4 `py-fuzzy <query> <choice> [choice...]`

Finds the best fuzzy match by Levenshtein ratio.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-fuzzy kittn kitten sitting knitting bitten
```

Example output:

```text
kitten    91
```

The score is an integer from `0` to `100`, where higher is more similar.

Library functions:

```rust
python_ops::levenshtein(a, b)
python_ops::fuzzy_ratio(a, b)
python_ops::best_fuzzy_match(query, &choices)
```

---

### 9.5 `py-identify <string>`

Identifies a simple string shape.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-identify test@example.com
```

Possible output enum variants:

```text
Email
Uuid
BitcoinAddressLike
Unknown
```

Library functions:

```rust
python_ops::identify_string(input)
```

Enum:

```rust
python_ops::IdentifierKind
```

---

### 9.6 `py-human-size <bytes>`

Renders a byte count as a human-friendly size.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-human-size 123456789
```

Example output:

```text
117.7 MiB
```

Library function:

```rust
python_ops::natural_size(bytes)
```

Related function:

```rust
python_ops::human_duration(seconds, future)
```

---

### 9.7 `py-validate-user <name> <age>`

Validates a simple user record.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-validate-user Alice 42
```

Valid conditions:

- Name must not be empty after trimming.
- Age must be between `0` and `150`, inclusive.

Example output:

```text
name=Alice age=42
```

Invalid example:

```powershell
.\target\debug\secure_diskmanager_ops.exe py-validate-user Alice 999
```

Expected error:

```text
error: invalid input: age must be in range 0..=150
```

Library items:

```rust
python_ops::ValidatedUser
python_ops::validate_user(name, age)
python_ops::parse_bounded_i32(input, min, max)
```

---

### 9.8 `py-tensor-demo`

Runs a tiny matrix multiplication demo equivalent to the Python `torch_tensor_demo.py` intent.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-tensor-demo
```

Example output shape:

```text
a=[1.0, 2.0, 3.0, 4.0]
c=[3.0, 3.0, 7.0, 7.0]
sum=20
```

Library type and methods:

```rust
python_ops::Matrix::new(rows, cols, data)
python_ops::Matrix::ones_like(&other)
matrix.get(row, col)
matrix.matmul(&rhs)
matrix.sum()
```

---

### 9.9 `py-chart-svg <out.svg>`

Writes a small SVG line chart.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-chart-svg demo_chart.svg
```

Output:

```text
wrote demo_chart.svg
```

Open the SVG in a browser or image viewer.

Library functions:

```rust
python_ops::line_chart_svg(title, labels, values, width, height)
python_ops::bar_chart_html(title, labels, values)
python_ops::ascii_plot(values, height)
python_ops::waffle_chart_text(values, rows)
python_ops::density_grid(points, width, height)
```

---

### 9.10 `py-pdf-summary <file.pdf>`

Runs a rough std-only PDF summary.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-pdf-summary document.pdf
```

Output format:

```text
Pages: 3
First text: some extracted literal string text...
```

Important limitations:

- This is not a full PDF engine.
- It guesses pages and extracts literal strings from PDF bytes.
- It will not reliably decode complex fonts, scanned images, OCR-only PDFs, or heavily compressed object streams.

Library functions:

```rust
python_ops::summarize_pdf_rough(path, max_chars)
python_ops::extract_pdf_literal_strings(text)
```

---

### 9.11 `py-greet [name]`

Simple greeting callback equivalent to the Gradio/Typer demos.

```powershell
.\target\debug\secure_diskmanager_ops.exe py-greet
.\target\debug\secure_diskmanager_ops.exe py-greet Wofl
```

Example output:

```text
Hello, Wofl!
```

Library function:

```rust
python_ops::greet(name)
```

---

## 10. Blocked CLI commands

These command names are recognized but intentionally blocked:

```text
selfdelete
kill-switch
stealth-mailer
tor-dispatch
mac-spoof
stealth-tunnel
```

Running one returns an error like:

```text
error: blocked operation: this operationally stealthy or destructive command is intentionally not ported
```

This is deliberate. These are not missing features or compile failures.

---

## 11. Example programs

### 11.1 `examples/demo.rs`

Build:

```powershell
cargo build --example demo
```

Run:

```powershell
.\target\debug\examples\demo.exe
```

Expected behaviour:

1. Creates a temporary demo directory.
2. Writes a small text file.
3. Calculates immediate disk usage.
4. Calculates entropy.
5. Searches for `.txt` files.
6. Compresses and decompresses the file.
7. Verifies byte-for-byte roundtrip equality.
8. Writes a 32-byte random keyfile.
9. Cleans up the temporary demo directory.

Expected output resembles:

```text
demo dir immediate file bytes before compression: 64
input entropy: 4.356898
search hits:
C:\Users\owner\AppData\Local\Temp\secure_diskmanager_ops_demo_...\demo_input.txt | Entropy: 4.356898

wrote demo keyfile: C:\Users\owner\AppData\Local\Temp\secure_diskmanager_ops_demo_...\demo.key
compression roundtrip OK: ...\demo_input.txt -> ...\demo_roundtrip.txt
```

If you see this, the core library path is functioning.

### 11.2 `examples/python_ops_demo.rs`

Build:

```powershell
cargo build --example python_ops_demo
```

Run:

```powershell
.\target\debug\examples\python_ops_demo.exe
```

Expected behaviour:

1. Prints number of represented Python demos.
2. Runs `python_ops::demo_report()`.
3. Generates an SVG chart string and prints its byte length.
4. Prints a small table derived from `DataRow` values.

---

## 12. Using it as a Rust library

The `.rlib` file in `target` is not meant to be run directly. It is produced so other Rust crates can link to this crate.

### 12.1 Add as a path dependency

In another Rust project’s `Cargo.toml`:

```toml
[dependencies]
secure_diskmanager_ops = { path = "../secure_diskmanager_ops_rust" }
```

Adjust the relative path to wherever this crate lives.

### 12.2 Basic library example

```rust
use secure_diskmanager_ops::{disk_management, entropy, rng};

fn main() -> secure_diskmanager_ops::Result<()> {
    let bytes = disk_management::get_disk_usage(".")?;
    println!("immediate file bytes: {bytes}");

    let key = rng::get_random_bytes(32)?;
    println!("key bytes: {}", key.len());

    let h = entropy::calculate_entropy("hello hello hello");
    println!("entropy: {h:.4}");

    Ok(())
}
```

### 12.3 Error handling

Most functions return:

```rust
secure_diskmanager_ops::Result<T>
```

which is an alias for:

```rust
std::result::Result<T, secure_diskmanager_ops::SdmError>
```

Error enum variants:

```rust
SdmError::Io(std::io::Error)
SdmError::InvalidInput(String)
SdmError::InvalidFormat(String)
SdmError::Compression(String)
SdmError::Crypto(String)
SdmError::UnsupportedPlatform(&'static str)
SdmError::Blocked(&'static str)
SdmError::CommandFailed { program: String, status: Option<i32> }
```

Recommended calling style:

```rust
fn main() -> secure_diskmanager_ops::Result<()> {
    // call crate functions with ?
    Ok(())
}
```

---

## 13. Library module reference

### 13.1 `disk_management`

Purpose: immediate-directory disk usage.

Public API:

```rust
pub fn get_disk_usage(path: impl AsRef<Path>) -> Result<u64>
```

Details:

- Reads the immediate directory entries at `path`.
- Adds sizes of regular files only.
- Does not recurse into subdirectories.

Use when you want the original C++ behaviour exactly.

---

### 13.2 `entropy`

Purpose: Shannon entropy calculations.

Public API:

```rust
pub fn calculate_entropy_bytes(data: &[u8]) -> f64
pub fn calculate_entropy(content: &str) -> f64
pub fn calculate_file_entropy(path: impl AsRef<Path>) -> Result<f64>
```

Use cases:

- Quick randomness/compression/encryption-like signal.
- Search result scoring.
- File triage.

Caution: entropy alone does not prove encryption, compression, or maliciousness. It is only a statistical hint.

---

### 13.3 `rng`

Purpose: OS randomness helpers.

Public API:

```rust
pub fn initialize() -> Result<()>
pub fn get_random_bytes(size: usize) -> Result<Vec<u8>>
pub fn get_random_64() -> Result<u64>
pub fn get_random_in_range(min: u64, max: u64) -> Result<u64>
pub fn write_keyfile(path: impl AsRef<Path>, num_bytes: usize) -> Result<()>
```

Details:

- Uses the `getrandom` crate.
- `initialize()` performs a 1-byte probe.
- `write_keyfile()` creates the file, writes random bytes, and syncs it.

---

### 13.4 `key_manager`

Purpose: simple local keyfile directory management.

Public API:

```rust
pub fn get_key_directory() -> Result<PathBuf>
pub fn generate_keyfile(name: &str, size: usize) -> Result<PathBuf>
pub fn delete_keyfile(name: &str) -> Result<()>
pub fn load_keyfile(name: &str) -> Result<Vec<u8>>
pub fn list_keyfiles() -> Result<Vec<String>>
```

Details:

- Uses a local `./keys` directory.
- `generate_keyfile()` creates random bytes using `rng`.
- `delete_keyfile()` uses the pattern shredder.

---

### 13.5 `compression`

Purpose: legacy-compatible file compression formats.

Public API:

```rust
pub fn compress(input_path, output_path) -> Result<()>
pub fn decompress(input_path, output_path) -> Result<()>
pub fn compress_mt(input_path, output_path, chunk_size: usize) -> Result<()>
pub fn decompress_mt(input_path, output_path) -> Result<()>
pub fn compress_mt_enc(input_path, output_path, keyfile: Option<K>, mode: &str) -> Result<()>
pub fn decompress_mt_enc(input_path, output_path, keyfile: Option<K>) -> Result<()>
```

Formats:

1. Single legacy zlib format:

```text
[u32 original_len_le][zlib payload]
```

2. Chunked legacy format:

```text
[u32 chunks][u32 chunk_size][u64 original_size]
repeated [u32 stored_size][stored_bytes]
```

3. Chunked XOR-obfuscated format:

Same as chunked, but each stored chunk is XORed after compression.

Cautions:

- XOR obfuscation is not modern encryption.
- Single-file legacy format has a 4 GiB length field limit.
- This is for compatibility and local operator use, not a replacement for standard archive formats.

---

### 13.6 `encryption`

Purpose: legacy RC4 compatibility.

Public API:

```rust
pub fn initialize_sbox(key: &[u8]) -> Result<[u8; 256]>
pub fn rc4_encrypt_decrypt(data: &mut [u8], key: &[u8]) -> Result<()>
pub fn encrypt_file(file_path: impl AsRef<Path>, key: &str) -> Result<PathBuf>
pub fn decrypt_file(file_path: impl AsRef<Path>, key: &str) -> Result<PathBuf>
pub fn encrypt_file_with_keyfile(file_path, keyfile_path) -> Result<PathBuf>
pub fn decrypt_file_with_keyfile(file_path, keyfile_path) -> Result<PathBuf>
```

Output behaviour:

| Function | Output suffix |
|---|---|
| `encrypt_file` | `.enc` |
| `decrypt_file` | `.dec` |
| `encrypt_file_with_keyfile` | `.enc` |
| `decrypt_file_with_keyfile` | `.dec` |

Caution: RC4 is not recommended for new cryptographic designs.

---

### 13.7 `dpapi`

Purpose: Windows DPAPI XML protect/unprotect compatibility.

Public API:

```rust
pub fn encrypt_to_xml(plaintext: &str, out_path: impl AsRef<Path>) -> Result<()>
pub fn decrypt_from_xml(xml_path: impl AsRef<Path>) -> Result<String>
pub fn decrypt_script(xml_path: impl AsRef<Path>) -> Result<String>
pub fn execute_decrypted_script(script_contents: &str) -> Result<()>
```

Details:

- `encrypt_to_xml` and `decrypt_from_xml` work on Windows only.
- Non-Windows returns `UnsupportedPlatform`.
- `decrypt_script` is just an alias-style wrapper around decrypting XML.
- `execute_decrypted_script` returns `Blocked`.

---

### 13.8 `search`

Purpose: recursive file path searching and metadata export.

Types:

```rust
pub struct SearchResult {
    pub path: PathBuf,
    pub entropy: Option<f64>,
}

pub struct FileEntry {
    pub full_path: PathBuf,
    pub size: u64,
    pub modified_time_unix: u64,
}
```

Public API:

```rust
pub fn find_matching_files(root_path, pattern: &str, use_entropy: bool) -> Result<Vec<SearchResult>>
pub fn format_search_results(results: &[SearchResult]) -> String
pub fn ultra_fast_search(pattern: &str, root_path) -> Result<Vec<FileEntry>>
pub fn format_file_entries(results: &[FileEntry]) -> String
pub fn export_to_csv(results: &[FileEntry], out_file) -> Result<()>
```

Details:

- `find_matching_files` uses the `regex` crate against path strings.
- `ultra_fast_search` uses simple substring matching.
- Both recurse with `walkdir` and do not follow links.

---

### 13.9 `secure_deletion`

Purpose: best-effort local overwrite-and-remove for a single regular file.

Public API:

```rust
pub fn shred_file(path: impl AsRef<Path>, passes: usize) -> Result<()>
pub fn shred_file_simd_pattern(path: impl AsRef<Path>, passes: usize) -> Result<()>
```

Safety behaviour:

- Rejects symlinks.
- Rejects directories.
- Does not recurse.
- Uses `passes.max(1)`.

Practical limitation: no overwrite-based shredder can guarantee total erasure on SSDs, snapshots, journaling filesystems, cloud sync, backups, or copy-on-write storage.

---

### 13.10 `gpg_wrapper`

Purpose: small wrapper around the system `gpg` command.

Public API:

```rust
pub fn encrypt_file(filepath: impl AsRef<Path>, recipient_key_id: &str) -> Result<()>
pub fn decrypt_file(filepath: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()>
pub fn import_key(keyfile: impl AsRef<Path>) -> Result<()>
pub fn key_exists(key_id: &str) -> Result<bool>
```

Details:

- Requires `gpg` to be installed and available in `PATH`.
- Returns `CommandFailed` if `gpg` exits unsuccessfully.

---

### 13.11 `file_transfer`

Purpose: explicit local upload/download wrapper using the system `curl` command.

Public API:

```rust
pub fn upload(local_path, remote_url: &str, keyfile: Option<K>, max_retries: usize) -> Result<()>
pub fn download(remote_url: &str, local_path, keyfile: Option<K>, max_retries: usize) -> Result<()>
pub fn send_via_smtp_tor(...) -> Result<()>
pub fn send_via_ftp_tor(...) -> Result<()>
```

Details:

- `upload` uses visible, explicit, user-supplied destinations.
- `download` uses visible, explicit, user-supplied URLs.
- Optional keyfile applies XOR obfuscation/deobfuscation to local bytes.
- `send_via_smtp_tor` and `send_via_ftp_tor` return `Blocked`.

Caution: XOR obfuscation is not modern encryption.

---

### 13.12 `fs_monitor`

Purpose: polling file-system watcher.

Types:

```rust
pub enum FsAction {
    Created,
    Modified,
}

pub struct FsMonitor { ... }
```

Public API:

```rust
FsMonitor::new(path)
watcher.watch_directory(callback)
watcher.stop_watching()
```

Details:

- Polls every 2 seconds.
- Recurses under the watched path.
- Calls callback with `(PathBuf, FsAction)`.
- Stops automatically on drop.

Example:

```rust
use secure_diskmanager_ops::fs_monitor::{FsMonitor, FsAction};

fn main() -> secure_diskmanager_ops::Result<()> {
    let mut mon = FsMonitor::new(".");
    mon.watch_directory(|path, action| {
        println!("{action:?}: {}", path.display());
    })?;

    std::thread::sleep(std::time::Duration::from_secs(10));
    mon.stop_watching();
    Ok(())
}
```

---

### 13.13 `system_monitor`

Purpose: Linux `/proc` CPU and memory helpers.

Public API:

```rust
pub fn get_cpu_usage() -> Result<f64>
pub fn get_memory_stats() -> Result<(u64, u64)>
```

Details:

- Implemented for Linux only.
- `get_cpu_usage()` reads `/proc/stat`; first call returns `0.0` because a delta sample is needed.
- `get_memory_stats()` returns `(total_bytes, available_bytes)` from `/proc/meminfo`.
- Windows currently returns `UnsupportedPlatform`.

---

### 13.14 `process_services`

Purpose: service/process helpers and blocked cleanup/wipe equivalents.

Types:

```rust
pub struct ServiceInfo {
    pub raw_line: String,
}

pub struct Watchdog { ... }
```

Public API:

```rust
pub fn list_services() -> Result<Vec<ServiceInfo>>
pub fn start_service(service_name: &str) -> Result<()>
pub fn stop_service(service_name: &str) -> Result<()>
pub fn self_cleanup_run() -> Result<()>
pub fn trigger_kill_switch(full_wipe: bool) -> Result<()>
pub fn wipe_identities() -> Result<()>
pub fn wipe_configs() -> Result<()>
pub fn terminate_tunnels() -> Result<()>
pub fn self_delete() -> Result<()>
pub fn daemon_start(service_name: &str) -> Result<()>
pub fn daemon_stop(service_name: &str) -> Result<()>
pub fn daemon_install(service_name: &str) -> Result<()>
pub fn daemon_uninstall(service_name: &str) -> Result<()>
Watchdog::new()
watchdog.add_watched_process(name)
watchdog.start(callback)
watchdog.stop()
watchdog.is_process_running(name)
```

Platform behaviour:

| Function | Windows | Linux |
|---|---|---|
| `list_services` | `sc query state= all` | `systemctl list-units --type=service --all --no-pager` |
| `start_service` | `sc start` | `systemctl start` |
| `stop_service` | `sc stop` | `systemctl stop` |
| `Watchdog::is_process_running` | `tasklist` string check | `pgrep -x` |

Blocked functions:

- `self_cleanup_run`
- `trigger_kill_switch`
- `wipe_identities`
- `wipe_configs`
- `terminate_tunnels`
- `self_delete`
- `daemon_install`
- `daemon_uninstall`

---

### 13.15 `identity`

Purpose: safe identity/persona planning and directory switching, without MAC spoofing or browser profile mutation.

Types:

```rust
pub struct Persona {
    pub name: String,
    pub mac_address: String,
    pub gpg_key_id: String,
    pub gpg_key_file: String,
}

pub struct IdentitySwitcher { ... }
pub struct PersonaSwitcher { ... }
pub struct GhostIdentityManager { ... }
pub struct IdentityActivationPlan { ... }
pub struct ProfileRotator { ... }
```

Key APIs:

```rust
IdentitySwitcher::new(identity_base_path)
switcher.switch_to_identity(identity_name)
switcher.list_identities()

PersonaSwitcher::new()
switcher.set_persona_root(root_path)
switcher.list_personas()
switcher.activate_persona(name)
switcher.active_persona()

GhostIdentityManager::new()
manager.load_identities(directory)
manager.list_identities()
manager.switch_to_identity(identity_name)
manager.get_current_identity()

ProfileRotator::new(interface)
rotator.load_personas(config_file)
rotator.rotate_to_next()
rotator.get_current_persona()
rotator.apply_persona(persona)
```

Details:

- `IdentitySwitcher` creates an `active` symlink pointing at a selected identity directory.
- `PersonaSwitcher` records active persona state only.
- `GhostIdentityManager::switch_to_identity` returns an `IdentityActivationPlan`; it does not spoof MACs or mutate browser profiles.
- `ProfileRotator::apply_persona` imports/checks GPG keys but does not mutate MAC addresses.

Persona config format for `ProfileRotator::load_personas`:

```text
name mac_address gpg_key_id gpg_key_file
```

Example:

```text
work 02:11:22:33:44:55 ABCDEF1234567890 keys/work.asc
lab 02:aa:bb:cc:dd:ee 0123456789ABCDEF keys/lab.asc
```

---

### 13.16 `net_admin`

Purpose: safe network utility pieces and blocked identity mutation/tunnel pieces.

Types:

```rust
pub struct GhostStealthNet { ... }
pub struct GhostTunnel { ... }
```

Public API:

```rust
pub fn generate_random_mac() -> Result<String>
pub fn get_current_mac(interface_name: &str) -> Result<String>
pub fn spoof_mac(interface_name: &str, new_mac: &str) -> Result<()>
pub fn change_mac_address(interface_name: &str, new_mac: &str) -> Result<()>

GhostStealthNet::new()
net.enable_dns_cloak()
net.setup_proxy_tunnel(proxy_address)
net.heartbeat()

GhostTunnel::new()
tunnel.initialize(proxy_address, proxy_port, encryption_key)
tunnel.start_tunnel()
tunnel.stop_tunnel()
tunnel.is_running()

pub fn start_vpn_connection(profile_name: &str) -> Result<()>
pub fn is_vpn_active() -> Result<bool>
```

Behaviour:

- `generate_random_mac()` generates a locally administered unicast MAC-like string.
- `get_current_mac()` is implemented for Unix-like `/sys/class/net/<iface>/address` only.
- `spoof_mac()` and `change_mac_address()` return `Blocked`.
- `GhostTunnel::start_tunnel()` returns `Blocked`.
- VPN functions use explicit host tools (`rasdial` on Windows, `nmcli` on Linux).

---

### 13.17 `quantum`

Purpose: small mathematical helpers from the original quantum/plugin pair.

Types:

```rust
pub struct Complex64 { pub re: f64, pub im: f64 }
pub struct Qubit { pub alpha: Complex64, pub beta: Complex64 }
pub struct HilbertCoord { pub x: u32, pub y: u32, pub z: u32 }
```

Public API:

```rust
Complex64::new(re, im)
complex.conj()
complex.norm()
complex.abs()

Qubit::default()
Qubit::new(alpha, beta)
qubit.normalize()
qubit.as_array()
qubit.to_state_string()

pub fn get_pauli_x() -> [[Complex64; 2]; 2]
pub fn apply_gate(gate, state) -> [Complex64; 2]
pub fn encode_text_as_qubit(text: &str) -> Qubit
pub fn similarity(a: &Qubit, b: &Qubit) -> f64
pub fn tokenize(content: &str) -> Vec<String>
pub fn prime_indices(tokens: &[String]) -> Vec<u64>
pub fn to_hilbert_3d(index: u64) -> HilbertCoord
pub fn hash_file_to_hilbert(path: impl AsRef<Path>) -> Result<HilbertCoord>
```

Notes:

- This is a lightweight mathematical/helper layer, not a quantum simulator framework.
- `hash_file_to_hilbert` reads text, tokenizes by whitespace, sums squared prime token indices, and maps to a simple 3D coordinate.

---

### 13.18 `cli`

Purpose: parse old C++-style flags into a command context and produce a plan. This is separate from the modern subcommand CLI in `src/main.rs`.

Types:

```rust
pub struct CommandContext { ... }
pub enum GhostCommandAction {
    Help,
    ListCommands,
    Exit,
    Execute(String),
}
```

Public API:

```rust
pub fn parse_args<I, S>(args: I) -> Result<CommandContext>
pub fn dispatch_plan(ctx: &CommandContext) -> Vec<String>
pub fn ghost_banner() -> &'static str
pub fn ghost_command_list() -> &'static [&'static str]
pub fn help_text() -> &'static str
pub fn handle_ghost_user_command(input: &str) -> GhostCommandAction
pub fn execute_ghost_command_stub(command: &str) -> String
pub fn handle_stealthmailer_cli(args: &[String]) -> Result<Vec<String>>
```

`handle_stealthmailer_cli` returns `Blocked` for operational stealth-mailer routines.

---

### 13.19 `ghost_controller`

Purpose: safe controller-state equivalents for original ghost/stealth orchestration surfaces.

Types:

```rust
pub struct GhostAutoWipe { ... }
pub enum ControllerState { Idle, Initialized, Running, Stopped, Error(String) }
pub struct GhostModeController { ... }
```

Public API summary:

```rust
GhostAutoWipe::new(path, delay_seconds)
auto_wipe.arm()
auto_wipe.abort()

GhostModeController::new()
controller.initialize_stealth_ops()
controller.init()
controller.configure_watchdog(process_names)
controller.start_all()
controller.run()
controller.stop_all()
controller.check_status()
controller.trigger_kill_switch()
controller.install_daemon(service_name)
controller.uninstall_daemon(service_name)
controller.start_daemon(service_name)
controller.stop_daemon(service_name)
controller.start_tunneling(endpoint)
controller.stop_tunneling()
controller.rotate_identity()
controller.generate_random_mac()
```

Important:

- Destructive or stealth parts return `Blocked` through lower-level functions.
- `GhostAutoWipe::arm()` is blocked; timed deletion automation was not ported.
- `start_tunneling()` is blocked by `GhostTunnel::start_tunnel()`.

---

### 13.20 `payload_dispatcher`

Purpose: queue structure for original payload dispatch shape, with covert dispatch blocked.

Types:

```rust
pub struct PayloadJob {
    pub method: String,
    pub file_path: PathBuf,
    pub remote_name: String,
    pub target: String,
    pub user: String,
    pub password: String,
}
```

Public API:

```rust
pub fn queue_payload(job: PayloadJob)
pub fn dispatch_all(use_tor: bool, spoof_mac: bool, shred_after: bool) -> Vec<Result<()>>
```

Behaviour:

- If `use_tor` or `spoof_mac` is true, dispatch result is `Blocked`.
- Methods `ftp` and `smtp` call blocked Tor-specific send functions, so they also return `Blocked`.
- Explicit non-covert transfers should use `file_transfer::upload/download` directly.

---

### 13.21 `stealth_mailer`

Purpose: preserve source surface while blocking stealth mailer behaviours.

Types:

```rust
pub struct MailerConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub to_address: String,
    pub subject: String,
    pub body: String,
    pub attachment_path: Option<String>,
}
```

Public API:

```rust
pub fn send_payload_securely(config: &MailerConfig) -> Result<()>
pub fn verify_tor_connection() -> Result<bool>
pub fn decrypt_credentials(enc_file_path: &str) -> Result<(String, String)>
```

All return `Blocked` or blocked-equivalent results. Use audited, explicit mail APIs outside this crate for legitimate mail sending.

---

## 14. `python_ops` full library reference

`python_ops.rs` is the v0.2.0 expansion from the Python demos. It is std-only and intentionally lightweight.

### 14.1 Represented demos

```rust
pub fn represented_python_demos() -> &'static [&'static str]
```

Returns the list of Python demo filenames represented in Rust.

---

### 14.2 Date/time and humanization

Types:

```rust
pub struct SimpleDateTime {
    pub unix_seconds: i64,
    pub offset_minutes: i32,
}
```

API:

```rust
SimpleDateTime::now_with_offset(offset_minutes)
SimpleDateTime::tokyo_now()
dt.add_seconds(seconds)
dt.add_minutes(minutes)
dt.add_hours(hours)
dt.add_days(days)
dt.humanize_since(reference)
dt.iso8601()

pub fn human_duration(seconds: u64, future: bool) -> String
pub fn natural_size(bytes: u64) -> String
```

Python demos represented: `arrow_demo.py`, `pendulum_demo.py`, `humanize_demo.py`.

---

### 14.3 Mini JSON / benedict-style dot paths

Types:

```rust
pub enum MiniValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<MiniValue>),
    Object(BTreeMap<String, MiniValue>),
}
```

API:

```rust
MiniValue::object()
value.as_object_mut()
value.to_json_string()

pub fn escape_json(input: &str) -> String
pub fn set_dot_path(root: &mut MiniValue, path: &str, value: MiniValue) -> Result<()>
pub fn get_dot_path(root: &MiniValue, path: &str) -> Option<&MiniValue>
pub fn demo_json_value() -> MiniValue
```

Python demos represented: `benedict_demo.py`, `orjson_demo.py`.

---

### 14.4 HTML helpers

API:

```rust
pub fn extract_first_tag_text(html: &str, tag: &str) -> Option<String>
pub fn extract_tag_texts(html: &str, tag: &str) -> Vec<String>
pub fn strip_tags(html_fragment: &str) -> String
pub fn html_unescape_basic(input: &str) -> String
```

Python demo represented: `bs4_demo.py`.

Limit: simple tag extraction only, not a full HTML parser.

---

### 14.5 Plain HTTP helpers

API:

```rust
pub fn build_http_get_request(host: &str, path: &str) -> String
pub fn plain_http_get(host: &str, port: u16, path: &str, timeout: Duration) -> Result<String>
```

Python demos represented: `httpx_demo.py`, `requests_demo.py`.

Limit: plain HTTP only. No TLS, redirects, cookies, streaming, proxy stack, or async client.

---

### 14.6 Data records and validation

Types:

```rust
pub struct PointRecord {
    pub x: f64,
    pub y: f64,
    pub meta: BTreeMap<String, String>,
}

pub struct ValidatedUser {
    pub name: String,
    pub age: i32,
}
```

API:

```rust
PointRecord::new(x, y)
point.with_meta(key, value)

pub fn validate_user(name, age) -> Result<ValidatedUser>
pub fn parse_bounded_i32(input, min, max) -> Result<i32>
```

Python demos represented: `dataclasses_demo.py`, `pydantic_demo.py`, `pyinputplus_demo.py`.

---

### 14.7 Mini config

Type:

```rust
pub struct MiniConfig { ... }
```

API:

```rust
MiniConfig::parse_toml_like(input)
config.with_env_prefix(prefix)
config.set_env(section)
config.get(key)
```

Python demo represented: `dynaconf_demo.py`.

Details:

- Parses simple TOML-like sections and `key = value` lines.
- Supports environment prefix/section lookup shape.
- Not a full TOML parser.

---

### 14.8 Deterministic faker

Types:

```rust
pub struct FakePerson {
    pub name: String,
    pub email: String,
    pub address: String,
}

pub struct DeterministicFaker { ... }
```

API:

```rust
DeterministicFaker::new(seed)
faker.person()
```

Python demo represented: `faker_demo.py`.

The output is deterministic for a given seed.

---

### 14.9 Ping/greeting/server helpers

API:

```rust
pub fn ping_json() -> &'static str
pub fn ping_text() -> &'static str
pub fn greet(name: &str) -> String
pub fn serve_ping_once(addr: &str) -> Result<()>
```

Python demos represented: `fastapi_demo.py`, `flask_demo.py`, `gradio_demo.py`, `typer_demo.py`.

`serve_ping_once` is a tiny one-shot plain TCP demo helper, not a production web server.

---

### 14.10 Fuzzy matching

API:

```rust
pub fn levenshtein(a: &str, b: &str) -> usize
pub fn fuzzy_ratio(a: &str, b: &str) -> u8
pub fn best_fuzzy_match(query: &str, choices: &[&str]) -> Option<(&str, u8)>
```

Python demo represented: `fuzzywuzzy_demo.py`.

---

### 14.11 Chart/text visualization helpers

API:

```rust
pub fn line_chart_svg(title, labels, values, width, height) -> Result<String>
pub fn bar_chart_html(title, labels, values) -> Result<String>
pub fn ascii_plot(values, height) -> String
pub fn waffle_chart_text(values, rows) -> String
pub fn density_grid(points, width, height) -> Vec<Vec<u32>>
```

Python demos represented: `cutecharts_demo.py`, `pyecharts_demo.py`, `plotext_demo.py`, `pywaffle_demo.py`, `datashader_demo.py`, `holoviews_demo.py`, `lightningchart_demo.py`.

Limit: lightweight generated SVG/HTML/text/grid data only. No browser renderer or licensed charting engine is embedded.

---

### 14.12 Logging/debug/table helpers

Types:

```rust
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
```

API:

```rust
pub fn log_line(level: LogLevel, message: &str) -> String
pub fn append_log_line(path, level, message, rotate_after_bytes) -> Result<()>
pub fn icecream_debug<T: Debug>(label: &str, value: &T) -> String
pub fn rich_table(title, headers, rows) -> String
```

Python demos represented: `logging_demo.py`, `loguru_demo.py`, `icecream_demo.py`, `rich_demo.py`.

---

### 14.13 Dataframe-style helpers

Type:

```rust
pub struct DataRow {
    pub x: i64,
    pub y: i64,
}
```

API:

```rust
row.z()
pub fn dataframe_assign_product(rows: &[DataRow]) -> Vec<(i64, i64, i64)>
pub fn dataframe_to_table(rows: &[DataRow]) -> String
```

Python demo represented: `pandas_demo.py`.

---

### 14.14 Matrix/tensor helpers

Type:

```rust
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}
```

API:

```rust
Matrix::new(rows, cols, data)
Matrix::ones_like(&other)
matrix.get(row, col)
matrix.matmul(&rhs)
matrix.sum()
```

Python demo represented: `torch_tensor_demo.py`.

Limit: tiny dense matrix helper only. No GPU, autograd, tensors, or neural network framework.

---

### 14.15 PDF/image/OCR helpers

Types:

```rust
pub struct PdfSummary {
    pub page_count_guess: usize,
    pub first_text: String,
}
```

API:

```rust
pub fn summarize_pdf_rough(path, max_chars) -> Result<PdfSummary>
pub fn extract_pdf_literal_strings(text: &str) -> String
pub fn demo_image_svg() -> Vec<u8>
pub fn tesseract_status_message() -> &'static str
```

Python demos represented: `pdfplumber_demo.py`, `pymupdf_demo.py`, `pillow_demo.py`, `pytesseract_demo.py`.

Limits:

- PDF summary is rough and std-only.
- SVG image demo is not a full Pillow replacement.
- OCR engine execution is not embedded.

---

### 14.16 Identifier detection

Enum:

```rust
pub enum IdentifierKind {
    Email,
    Uuid,
    BitcoinAddressLike,
    Unknown,
}
```

API:

```rust
pub fn identify_string(input: &str) -> IdentifierKind
```

Python demo represented: `pywhat_demo.py`.

---

### 14.17 Stdout/lint helpers

Types:

```rust
pub struct PythonLintIssue {
    pub line: usize,
    pub message: String,
}
```

API:

```rust
pub fn noisy_lines() -> Vec<&'static str>
pub fn capture_noisy() -> String
pub fn mini_python_lint(source: &str) -> Vec<PythonLintIssue>
```

Python demos represented: `redirect_stdout_demo.py`, `ruff_demo.py`.

`mini_python_lint` is intentionally tiny and heuristic. It is not Ruff.

---

### 14.18 Schedule/retry/flow/watch helpers

Types:

```rust
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub modified: SystemTime,
}
```

API:

```rust
pub fn run_fixed_schedule(interval, ticks, job) -> Result<usize>
pub fn retry_fixed(attempts, wait, op) -> Result<T, E>
pub fn extract_task() -> Vec<i32>
pub fn transform_task(xs: &[i32]) -> Vec<i32>
pub fn etl_flow() -> Vec<i32>
pub fn watch_file_mtime(path, timeout, poll_every) -> Result<Option<FileChangeEvent>>
```

Python demos represented: `schedule_demo.py`, `tenacity_demo.py`, `prefect_demo.py`, `watchdog_demo.py`.

---

### 14.19 Sentence similarity

API:

```rust
pub fn bag_of_words_vector(sentence: &str) -> BTreeMap<String, f64>
pub fn cosine_similarity_sparse(a, b) -> f64
pub fn sentence_similarity_scores(query, sentences) -> Vec<(&str, f64)>
```

Python demo represented: `sentence_transformers_demo.py`.

Limit: bag-of-words cosine similarity only. No transformer model, embeddings model, tokenizer, or neural inference.

---

### 14.20 Geometry helpers

Type:

```rust
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
```

API:

```rust
pub fn polygon_area(points: &[Point2D]) -> f64
pub fn polygon_contains_point(poly: &[Point2D], point: Point2D) -> bool
```

Python demo represented: `shapely_demo.py`.

---

### 14.21 External service placeholders

API:

```rust
pub fn external_service_blocked(service: &'static str) -> Result<()>
```

Used for demos that require external credentials, browsers, OCR engines, or paid/licensed services:

- OpenAI API
- Playwright
- Selenium
- Tesseract OCR
- LightningChart-style renderer

These should be integrated at the application layer with explicit user configuration.

---

## 15. Workflows

### 15.1 Verify the crate after extraction

```powershell
cd C:\github\secure-disk-manager-pro\secure_diskmanager_ops_rust
cargo clean
cargo build --bin secure_diskmanager_ops
cargo build --example demo
cargo build --example python_ops_demo
cargo run --example demo
cargo run --example python_ops_demo
cargo run --bin secure_diskmanager_ops -- help
```

### 15.2 Compression roundtrip

```powershell
Set-Content .\sample.txt "hello from secure diskmanager ops"
.\target\debug\secure_diskmanager_ops.exe compress .\sample.txt .\sample.txt.z
.\target\debug\secure_diskmanager_ops.exe decompress .\sample.txt.z .\sample.roundtrip.txt
Compare-Object (Get-Content .\sample.txt) (Get-Content .\sample.roundtrip.txt)
```

No output from `Compare-Object` means the files match as text lines.

For byte comparison:

```powershell
$A = [System.IO.File]::ReadAllBytes((Resolve-Path .\sample.txt))
$B = [System.IO.File]::ReadAllBytes((Resolve-Path .\sample.roundtrip.txt))
[System.Linq.Enumerable]::SequenceEqual($A, $B)
```

Expected:

```text
True
```

### 15.3 Search source files and export metadata

```powershell
.\target\debug\secure_diskmanager_ops.exe search . "\.rs$" --entropy
.\target\debug\secure_diskmanager_ops.exe ultra-search . Cargo --csv cargo_hits.csv
```

### 15.4 Generate keyfile and use RC4 compatibility transform

```powershell
Set-Content .\secret.txt "demo secret"
.\target\debug\secure_diskmanager_ops.exe rng-keyfile .\legacy.key 32
.\target\debug\secure_diskmanager_ops.exe rc4-encrypt-keyfile .\secret.txt .\legacy.key
.\target\debug\secure_diskmanager_ops.exe rc4-decrypt-keyfile .\secret.txt.enc .\legacy.key
```

This produces:

```text
secret.txt.enc
secret.txt.enc.dec
```

Again: this is legacy compatibility, not modern crypto guidance.

### 15.5 Windows DPAPI roundtrip

```powershell
.\target\debug\secure_diskmanager_ops.exe dpapi-protect "hello DPAPI" .\dpapi_secret.xml
.\target\debug\secure_diskmanager_ops.exe dpapi-unprotect .\dpapi_secret.xml
```

Expected plaintext:

```text
hello DPAPI
```

### 15.6 Python-demo report and chart

```powershell
.\target\debug\secure_diskmanager_ops.exe py-demo-report
.\target\debug\secure_diskmanager_ops.exe py-chart-svg .\demo_chart.svg
Invoke-Item .\demo_chart.svg
```

---

## 16. Troubleshooting

### 16.1 “I only see `libsecure_diskmanager_ops.rlib`.”

That means the library crate compiled. It does not mean the CLI binary was built.

Run:

```powershell
cargo build --bin secure_diskmanager_ops
```

Expected:

```text
target\debug\secure_diskmanager_ops.exe
```

### 16.2 “I only see `demo.exe`.”

You built only the example:

```powershell
cargo build --example demo
```

Build the real CLI:

```powershell
cargo build --bin secure_diskmanager_ops
```

### 16.3 “The only `.exe` is under `target\debug\build\...`.”

That is a dependency build script, not the app. Ignore it.

Search for all executables:

```powershell
Get-ChildItem -Path .\target -Recurse -Filter *.exe | Select-Object -ExpandProperty FullName
```

Useful paths are:

```text
target\debug\secure_diskmanager_ops.exe
target\debug\examples\demo.exe
target\debug\examples\python_ops_demo.exe
```

### 16.4 “`cargo build --bin secure_diskmanager_ops` says no bin target.”

Check `Cargo.toml` contains:

```toml
[[bin]]
name = "secure_diskmanager_ops"
path = "src/main.rs"
```

Check `src\main.rs` exists:

```powershell
Test-Path .\src\main.rs
```

If either is missing, you are in an older extracted archive.

### 16.5 “PowerShell regex search gives weird results.”

Use quotes:

```powershell
.\target\debug\secure_diskmanager_ops.exe search . "\.rs$"
```

For paths containing spaces, quote paths too:

```powershell
.\target\debug\secure_diskmanager_ops.exe search "C:\My Folder" "\.txt$"
```

### 16.6 “DPAPI says unsupported platform.”

DPAPI functions only work on Windows. On Linux/macOS they intentionally return `UnsupportedPlatform`.

### 16.7 “GPG wrapper fails.”

Check GPG is installed:

```powershell
gpg --version
```

Check the key exists:

```powershell
gpg --list-keys YOUR_KEY_ID
```

### 16.8 “File transfer wrapper fails.”

Check curl:

```powershell
curl --version
```

Remember: file transfer is library-only in this version, not a CLI command.

### 16.9 “Shred did not recover space immediately.”

Filesystem metadata, antivirus, indexing, cloud sync, recycle bin policy, delayed writes, or snapshots can affect visible state. The function overwrites and removes a single regular file; it does not manage storage backends.

### 16.10 “I got `blocked operation`.”

That is expected for intentionally unported operations such as stealth mailer, Tor dispatch, kill-switch wiping, self-delete, MAC spoofing, stealth tunnel, and hidden script execution.

---

## 17. Current known limitations

1. The CLI does not expose every library function.
2. The crate is not a modern cryptography toolkit.
3. RC4 and XOR paths are compatibility/obfuscation only.
4. PDF extraction is rough and not OCR-capable.
5. HTML extraction is simple and not a full parser.
6. Sentence similarity is bag-of-words, not transformer embeddings.
7. Matrix support is tiny dense matrix math, not Torch.
8. Service/process helpers depend on OS commands.
9. DPAPI is Windows-only.
10. Linux system monitor helpers are Linux-only.
11. Shredding cannot guarantee erasure across SSD wear-levelling, journaling, snapshots, backups, or copy-on-write storage.
12. Stealth/destructive routines are deliberately blocked.

---

## 18. Recommended next development steps

For the project to become more useful as an OS operator toolkit, the next practical improvements would be:

1. Add a real CLI parser such as `clap` for better help, validation, and subcommands.
2. Add integration tests for every CLI command.
3. Add Windows implementations for CPU/memory stats.
4. Add optional `serde` support for JSON output.
5. Add SHA-256/hash commands for file verification.
6. Add non-destructive dry-run modes for file operations.
7. Add a `doctor` command that checks Rust version, cargo targets, `gpg`, `curl`, platform support, and output paths.
8. Add a release-packaging script that copies only the final `.exe`, README/manual, and license into a clean `dist/` folder.

---

## 19. Quick command crib sheet

```powershell
# Build
cargo build --bin secure_diskmanager_ops
cargo build --release --bin secure_diskmanager_ops
cargo build --example demo
cargo build --example python_ops_demo

# Run help
.\target\debug\secure_diskmanager_ops.exe help
cargo run --bin secure_diskmanager_ops -- help

# Core commands
.\target\debug\secure_diskmanager_ops.exe disk-usage .
.\target\debug\secure_diskmanager_ops.exe search . "\.rs$" --entropy
.\target\debug\secure_diskmanager_ops.exe ultra-search . Cargo --csv hits.csv
.\target\debug\secure_diskmanager_ops.exe entropy .\Cargo.toml
.\target\debug\secure_diskmanager_ops.exe rng-keyfile .\my.key 32
.\target\debug\secure_diskmanager_ops.exe compress input.bin input.bin.z
.\target\debug\secure_diskmanager_ops.exe decompress input.bin.z restored.bin
.\target\debug\secure_diskmanager_ops.exe compress-mt input.bin input.bin.mtz
.\target\debug\secure_diskmanager_ops.exe decompress-mt input.bin.mtz restored.bin
.\target\debug\secure_diskmanager_ops.exe compress-mt-enc input.bin input.bin.mte my.key fast
.\target\debug\secure_diskmanager_ops.exe decompress-mt-enc input.bin.mte restored.bin my.key
.\target\debug\secure_diskmanager_ops.exe rc4-encrypt input.txt "legacy-key"
.\target\debug\secure_diskmanager_ops.exe rc4-decrypt input.txt.enc "legacy-key"
.\target\debug\secure_diskmanager_ops.exe dpapi-protect "hello" secret.xml
.\target\debug\secure_diskmanager_ops.exe dpapi-unprotect secret.xml
.\target\debug\secure_diskmanager_ops.exe shred scratch.bin 3

# Python-demo commands
.\target\debug\secure_diskmanager_ops.exe py-demo-list
.\target\debug\secure_diskmanager_ops.exe py-demo-report
.\target\debug\secure_diskmanager_ops.exe py-html h1 "<h1>Hello</h1>"
.\target\debug\secure_diskmanager_ops.exe py-fuzzy kittn kitten sitting knitting bitten
.\target\debug\secure_diskmanager_ops.exe py-identify test@example.com
.\target\debug\secure_diskmanager_ops.exe py-human-size 123456789
.\target\debug\secure_diskmanager_ops.exe py-validate-user Alice 42
.\target\debug\secure_diskmanager_ops.exe py-tensor-demo
.\target\debug\secure_diskmanager_ops.exe py-chart-svg chart.svg
.\target\debug\secure_diskmanager_ops.exe py-pdf-summary document.pdf
.\target\debug\secure_diskmanager_ops.exe py-greet Wofl
```

---

## 20. Final orientation

Use `secure_diskmanager_ops.exe` when you want quick command-line operations.

Use `libsecure_diskmanager_ops.rlib` indirectly by adding this crate as a Rust dependency when you want to embed the operators into a larger OS/project.

Use the examples to confirm basic health:

```powershell
cargo run --example demo
cargo run --example python_ops_demo
```

Use `FUNCTION_MAP.md` and `PYTHON_DEMO_FUNCTION_MAP.md` when you want to trace a Rust function back to the original C++ or Python source intent.

