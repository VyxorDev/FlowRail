# FlowRail

**Your desktop has a taskbar. It deserves a signal track.**

FlowRail is a free, privacy-first, open-source Windows activity rail written entirely in Rust. Its **vertical signal track** is an original Windows-first UI, not a Dynamic Island clone.

> **v0.1 prototype:** draggable borderless app, not yet an edge-docked background utility. Fullscreen detection, tray, media controls, notifications, and plugins are future milestones.

## Try FlowRail — no developer tools required

You do **not** need Rust, Cargo, Visual Studio, or C++ Build Tools to run FlowRail.

1. Open [FlowRail Windows builds](https://github.com/VyxorDev/FlowRail/actions/workflows/ci.yml).
2. Open the latest successful **Windows Rust CI** run.
3. Under **Artifacts**, download **FlowRail-Windows-x64**.
4. Extract the ZIP and run `FlowRail.exe`.

Each build includes a SHA-256 checksum in `SHA256SUMS.txt). FlowRail is an early prototype and is not yet code-signed or distributed as an installer.

## Build from source (contributors)

Building locally requires Windows 11, Rust 1.95.0 (pinned in rust-toolchain.toml), and the Visual Studio C++ Build Tools with the MSVC linker and Windows SDK.

    git clone https://github.com/VyxorDev/FlowRail.git
    cd FlowRail
    cargo run

Build an optimized executable:

    cargo build --release

No administrator rights are required to run the app. Settings live locally at %APPDATA%\FlowRail\settings.json.

## Current features

- Original graphite/mint signal-track interface with a custom startup-applied theme
- Focus session start/pause/resume/reset
- Prioritized activity stream with Quiet filtering
- Real Win32 system uptime and local app uptime
- Explorer, Windows Settings and Task Manager quick actions
- Local Quiet and Compact preferences; unit tests and Windows CI
- Event-driven repaint policy: 1 Hz for the running timer, 60-second idle refresh
- Low-power GPU adapter preference for hybrid laptops

## Performance and UX are product requirements

FlowRail is designed to stay out of the way: no busy loops, unnecessary polling, always-on blur, animated backgrounds, telemetry, or cloud dependency. It requests a low-power graphics adapter and redraws only for user/OS events plus the minimum timer refresh needed.

Initial CPU, memory and startup budgets are **targets to measure**, not performance claims. See [Performance & Design](docs/PERFORMANCE_AND_DESIGN.md) for the budgets and a Windows sampling script.

The application contains no webviews, JavaScript, cloud backend, tracking, or advertisements. Never disable Windows security protections to run FlowRail.

Developer checks: cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo test, cargo build --release.

See [Architecture](docs/ARCHITECTURE.md), [Roadmap](docs/ROADMAP.md), [Performance & Design](docs/PERFORMANCE_AND_DESIGN.md), SECURITY.md, and CONTRIBUTING.md.

MIT © 2026 VyxorDev. Independent project, not affiliated with Microsoft or Apple.
