# Blemmcord Lite

A Rust-powered Windows desktop shell for the official Discord web client, built with Tauri 2 and Microsoft Edge WebView2.

## Phase 1 status

- [x] Project scaffold and application identity
- [x] Tauri 2 + Rust application entry point
- [x] WebView2 window opens the official Discord web app
- [x] Dark Discord-inspired startup/fallback page
- [x] Basic Rust runtime information command
- [x] Windows NSIS (.exe) and MSI installer targets enabled
- [ ] Local settings persistence
- [ ] Build and test the Windows installers
- [ ] Memory benchmarking against the normal Discord desktop client

## What gets built

The Windows release build is configured to produce:

- **NSIS installer (.exe)** — a normal Windows setup wizard.
- **MSI installer (.msi)** — a Windows Installer package.
- **Application executable** — the actual Blemmcord Lite program.

The generated files will be under `src-tauri/target/release/bundle/` after a successful Windows build. The raw executable is under `src-tauri/target/release/`.

## Requirements

- Windows 10 or 11
- Rust stable toolchain (install from https://rustup.rs/)
- Microsoft C++ Build Tools with the MSVC toolchain and Windows SDK
- Microsoft Edge WebView2 Runtime
- Node.js LTS and npm

## Run in development

From the repository root in PowerShell:

```powershell
npm install
npm run dev
```

The first Rust build may take several minutes while Cargo downloads and compiles dependencies.

## Build the Windows EXE and installers

Run this on a Windows PC:

```powershell
npm install
npm run build
```

After a successful build, look in:

- `src-tauri/target/release/blemmcord-lite.exe`
- `src-tauri/target/release/bundle/nsis/` for the setup EXE
- `src-tauri/target/release/bundle/msi/` for the MSI installer

The installer build is configured in `src-tauri/tauri.conf.json`. Building on Windows is required to produce Windows installers.

## Account safety and compatibility

Blemmcord Lite opens `https://discord.com/app` as a top-level WebView2 page. It does not embed Discord in an iframe, extract account tokens, automate a user account, or call undocumented Discord endpoints.

Discord's web client remains responsible for login, servers, messages, voice, video, and other supported web-client features. Since WebView2 is Chromium-based, lower memory usage is not guaranteed; benchmark it on the target PC before drawing conclusions. This third-party wrapper is not an official Discord product and may need maintenance if Discord changes its web client or wrapper compatibility.

## Project layout

```
.
├── src/
│   └── index.html
├── src-tauri/
│   ├── capabilities/
│   │   └── default.json
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── build.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── .gitignore
├── package.json
└── README.md
```

## Roadmap

1. Establish the Tauri/WebView2 desktop shell.
2. Verify Discord login, navigation, messaging, attachments, and voice/video on Windows.
3. Add app-owned settings and lightweight runtime/memory diagnostics.
4. Benchmark idle and active memory usage against the official desktop client.
5. Finalize branding, icons, and installer testing.
