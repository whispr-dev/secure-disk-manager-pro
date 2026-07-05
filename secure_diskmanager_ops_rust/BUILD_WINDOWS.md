# Build on Windows

Run these commands from the `secure_diskmanager_ops_rust` crate root, the folder that contains `Cargo.toml` and `src/main.rs`.

```powershell
.\build_windows.ps1
```

Release build:

```powershell
.\build_windows.ps1 -Release
```

Expected debug executable:

```text
target\debug\secure_diskmanager_ops.exe
```

Expected demo executable:

```text
target\debug\examples\demo.exe
```

The `.rlib` file is the Rust library artifact. It is normal, but it is not the runnable application. Files under `target\debug\build\...` are dependency build-script executables and are not the application either.
