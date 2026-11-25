# Blemmcord Lite

A lightweight native Windows desktop client shell written in Rust. The interface uses **egui + wgpu** and does not embed Chromium, Electron, Node.js, Tauri, or WebView2.

## Phase 1: Native UI foundation

- [x] Native Rust application entry point
- [x] egui interface rendered with wgpu
- [x] Dark Discord-inspired layout
- [x] Server rail and selectable text-channel navigation
- [x] Native message composer prototype
- [x] Windows x64 CI build workflow
- [ ] Run and verify the first Windows build
- [ ] Add app settings and persistent preferences
- [ ] Measure memory usage on Windows

The current interface is a local UI prototype. It does **not** connect to Discord, sign in, or send messages yet.

## Build on Windows

Install:
- Rust stable (MSVC toolchain): https://rustup.rs/
- Visual Studio Build Tools with the Desktop development with C++ workload
- Windows 10/11 SDK
- Git

From the repository root in PowerShell:

```powershell
cargo run --manifest-path src-tauri/Cargo.toml
```

Build an optimized executable:

```powershell
cargo build --release --manifest-path src-tauri/Cargo.toml
```

The executable is:

```
src-tauri/target/release/blemmcord-lite.exe
```

A successful push to `main` will also trigger the Windows GitHub Actions build. The workflow uploads the executable as a downloadable build artifact.

## Architecture

- **Rust** owns application state and business logic.
- **egui** renders the native interface.
- **wgpu** provides GPU-backed rendering.
- No browser engine is embedded in the main application window.

## Discord connectivity and account safety

Blemmcord Lite will not extract session tokens, automate a normal Discord user account, or imitate undocumented/private Discord client endpoints. Discord's supported bot and OAuth2 APIs do not provide a complete replacement for the normal Discord user client. That means the current native shell is not yet a full Discord client; supported connectivity options must be designed separately.

This project is an independent third-party project and is not affiliated with or endorsed by Discord.

## Project layout

```
src-tauri/
  Cargo.toml
  src/
    main.rs
    lib.rs
`
```

## License

Project source is maintained by WebsiteDevelopmentStudios. Third-party dependencies retain their respective licenses.
