# Blemmcord Lite

A lightweight Windows desktop shell for the official Discord web client, built with Rust and Tauri 2.

## Phase 1 status

- [x] Project scaffold and application identity
- [x] Tauri 2 + Rust application entry point
- [x] WebView2 window configured to open the official Discord web app
- [x] Dark Discord-inspired desktop styling for the local startup/fallback page
- [x] Basic Rust runtime information command
- [ ] Local settings persistence
- [ ] Windows packaging and installer
- [ ] Memory benchmarking against the normal Discord desktop client

## Important compatibility and account-safety notes

Blemmcord Lite opens `https://discord.com/app` as a top-level WebView2 page. It does not embed Discord in an iframe, extract account tokens, automate a user account, or call undocumented Discord endpoints.

Discord's web client remains responsible for login, servers, messages, voice, video, and other supported web-client features. Since WebView2 is Chromium-based, lower memory usage is not guaranteed; measure it on the target PC before drawing conclusions. This third-party wrapper is not an official Discord product and may need maintenance if Discord changes its web client or wrapper compatibility.

## Requirements

- Windows 10 or 11
- Rust stable toolchain (install from https://rustup.rs/)
- Microsoft Edge WebView2 Runtime
- Node.js and npm
- Tauri CLI (installed through the project dependencies)

## Run in development

```powershell
npm install
npm run tauri dev
```

The first Rust build may take several minutes while Cargo downloads and compiles dependencies.

## Build

```powershell
npm run tauri build
```

Packaging is intentionally disabled in this first scaffold until the project has its final icon and installer settings.

## Project layout

```
.
├── src/
│   └── index.html
├── src-tauri/
│   ├── capabilities/
│   │   └── default.json
│   ├── src/
│   │   └── main.rs
│   ├── build.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── .gitignore
├── package.json
└── README.md
```

## Roadmap

1. Establish the Tauri/WebView2 shell.
2. Verify Discord login, navigation, messaging, attachments, and voice/video on Windows.
3. Add app-owned settings and lightweight runtime/memory diagnostics.
4. Benchmark idle and active memory usage against the official desktop client.
5. Add branding, icons, and a Windows installer.
