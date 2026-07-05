# secure_diskmanager_ops

Rust equivalents for the useful local-OS operators found in the uploaded `Secure_DiskManager_Pro` C++ archive.

## What was ported

This crate ports the matched C++ header/source module intent into safe Rust modules:

- disk usage
- zlib-compatible legacy file compression/decompression
- chunked compression/decompression, including the legacy XOR-obfuscated variant
- legacy RC4 file transform compatibility
- random/keyfile helpers
- local search, CSV export, entropy scoring, and Hilbert coordinate hashing
- secure single-file deletion operators
- identity/persona directory switching without MAC spoofing or browser-profile mutation
- GPG wrapper functions
- Windows DPAPI XML protect/unprotect when compiled on Windows
- filesystem monitor polling
- system/service/process monitoring helpers
- quantum/qubit helper functions from the plugin pair
- CLI parse/dispatch-plan helpers

## What was intentionally blocked

The source archive included modules labelled or implemented as stealth/Tor payload dispatch, MAC spoofing, encrypted traffic relay, hidden PowerShell execution, auto-wipe timers, kill-switch wipes, daemon persistence, and self-deleting executable behavior. Those are represented as explicit `SdmError::Blocked(...)` returns rather than operational code.

That keeps the crate useful for legitimate OS/module development without recreating covert exfiltration, stealth persistence, or destructive automation.

## Layout

```text
src/
  cli.rs                  CommandContext, arg parsing, dispatch plans, UI text
  compression.rs          FileCompressor*, chunked formats, XOR legacy variant
  disk_management.rs      get_disk_usage
  dpapi.rs                DPAPI XML protect/unprotect on Windows; script exec blocked
  encryption.rs           legacy RC4 compatibility transforms
  entropy.rs              Shannon entropy helpers
  file_transfer.rs        explicit curl-backed upload/download; Tor send blocked
  fs_monitor.rs           polling filesystem monitor
  ghost_controller.rs     safe GhostModeController shell and AutoWipe block
  gpg_wrapper.rs          gpg command wrapper
  identity.rs             identity/persona/profile switching primitives
  key_manager.rs          key directory/keyfile helpers
  net_admin.rs            random/current MAC helpers; spoof/tunnel blocked
  payload_dispatcher.rs   queue API; covert dispatch blocked
  process_services.rs     services, watchdog, blocked kill/self-delete
  quantum.rs              qubit, Pauli-X, Hilbert indexing
  rng.rs                  OS randomness helpers
  search.rs               SearchEngine and UltraFastSearch equivalents
  secure_deletion.rs      single-file shredding equivalents
  system_monitor.rs       Linux /proc CPU/memory equivalents
```

## Build

```bash
cargo build
```

On Windows, DPAPI support uses direct Win32 FFI bindings to Crypt32/Kernel32. On Linux, DPAPI calls return `UnsupportedPlatform` by design.


## Building the runnable `.exe`

This package is both:

1. a Rust library crate, producing artifacts such as `libsecure_diskmanager_ops.rlib`, and
2. a command-line executable, producing `secure_diskmanager_ops.exe` on Windows.

`cargo check` only type-checks. It does not create a runnable executable.

Build the executable with:

```powershell
cargo build
```

The debug executable should appear at:

```text
target\debug\secure_diskmanager_ops.exe
```

The optimized release executable should appear at:

```powershell
cargo build --release
```

```text
target\release\secure_diskmanager_ops.exe
```

Run the CLI help with:

```powershell
.\target\debug\secure_diskmanager_ops.exe help
```

The example binary, if built with `cargo build --example demo`, appears at:

```text
target\debug\examples\demo.exe
```

not inside `target\debug\build`.

## CLI examples

```powershell
.\target\debug\secure_diskmanager_ops.exe disk-usage .
.\target\debug\secure_diskmanager_ops.exe search . "\.rs$" --entropy
.\target\debug\secure_diskmanager_ops.exe compress input.bin input.bin.z
.\target\debug\secure_diskmanager_ops.exe decompress input.bin.z input.roundtrip.bin
.\target\debug\secure_diskmanager_ops.exe rng-keyfile my.key 32
```

## Minimal library usage

```rust
use secure_diskmanager_ops::{compression, disk_management, search};

fn main() -> secure_diskmanager_ops::Result<()> {
    let bytes = disk_management::get_disk_usage(".")?;
    println!("immediate file bytes: {bytes}");

    compression::compress("input.bin", "input.bin.z")?;
    compression::decompress("input.bin.z", "input.roundtrip.bin")?;

    let hits = search::find_matching_files(".", r"\.rs$", false)?;
    println!("{}", search::format_search_results(&hits));
    Ok(())
}
```

## Security notes

- `encryption.rs` ports RC4 only for compatibility with the original code. Do not use RC4 for new security-critical designs.
- `compression.rs` ports the legacy XOR-obfuscated compressor. That XOR path is not real encryption.
- `secure_deletion.rs` overwrites regular files and refuses symlinks, but SSD wear-leveling and journaling filesystems can still preserve old data outside application control.
