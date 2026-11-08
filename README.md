# FlowRail

**Your desktop has a taskbar. It deserves a signal track.**

FlowRail is a free, open-source, privacy-first **Windows activity rail** written entirely in Rust. It's **not** an Apple Dynamic Island replica: it's a user-controlled vertical signal track that surfaces actionable activity alongside your work.

> **Status: v0.1 prototype.** The initial version is a draggable, borderless app, not yet a system-tray background companion. Media, notifications, global shortcuts, edge docking, and extensions are future milestones.

## Principles

- Windows-first; original UI instead of copying another OS.
- All app and UI code in Rust; no Electron, JS, Node, or webview.
- No accounts, cloud backend, telemetry, or admin privileges.
- Local-first, small permissions, event-driven Windows integrations.
- Explicit controls and consideration for fullscreen apps.

## Run on Windows

Requires Windows 11, Rust **1.95.0** (pinned in `rust-toolchain.toml`), and MSVC Build Tools/Windows SDK.

```powershell
git clone https://github.com/VyxorDev/FlowRail.git
cd FlowRail
cargo run
```

For a release-mode local build:

```powershell
cargo build --release
.\target\release\flowrail.exe
```

Preferences are local in `%APPDATA%\FlowRail\settings.json`. No administrator rights required.

## Implemented prototype

- Original graphite/mint signal-track UI with draggable borderless window
- Focus session: start, pause, resume, reset
- Priority-ordered activity stream and quiet-mode filtering
- Real Windows system uptime via the Win32 API
- Explorer, Windows Settings, and Task Manager quick launch
- Persistent local Quiet and Compact preferences
- Rust unit tests and Windows GitHub Actions CI configuration

**Not yet implemented:** edge docking, tray, single instance, fullscreen detection, WinRT media sessions, notification access, clipboard history, plugin runtime, signed binaries. No one should mistake this prototype for a finished utility.

## Developer commands

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

See [architecture](docs/ARCHITECTURE.md), [roadmap](docs/ROADMAP.md), [security](SECURITY.md), and [contribution guide](CONTRIBUTING.md).

## License

MIT © 2026 VyxorDev. Independent project, not affiliated with Microsoft or Apple.
