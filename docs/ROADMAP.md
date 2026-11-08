# FlowRail roadmap

Items below describe **planned work**, not already delivered features.

## Milestone 0 — Initial Rust prototype (current)

- [x] Rust application with borderless draggable graphite/mint rail
- [x] Actual 25-minute focus timer with start/pause/resume/reset
- [x] Priority ordering, ID coalescing and Quiet filtering
- [x] Genuine Windows system uptime and safe predefined quick actions
- [x] Local JSON settings and Rust unit tests
- [x] MIT license, security policy, contribution guide, documentation and CI definition
- [x] Event-driven repaint schedule: 1 Hz focus timer, 60-second idle refresh
- [x] Low-power GPU adapter preference and startup-only visual style configuration
- [x] Performance budget, measurement script and premium UX contract
- [ ] Verify Windows CI and manual launch on a Windows 11 computer
- [ ] Commit a resolved Cargo.lock for reproducible application builds

## Milestone 1 — Resident Windows companion

- [ ] System tray icon with Open, Hide and Exit commands
- [ ] Snap the rail to a chosen monitor work area and respect per-monitor DPI
- [ ] Distinct collapsed, glance and expanded states on an edge-aligned signal spine
- [ ] Auto-hide/suppress during fullscreen games with no process injection
- [ ] Single-instance guard, clean teardown and optional launch on sign-in
- [ ] Suspend repaint timers and nonessential providers while hidden
- [ ] Correct tab order, accessible labels, high contrast and reduced motion
- [ ] Profile visible idle, hidden idle, animation and startup on real Windows 11 hardware

**Done when:** it behaves as a genuine unobtrusive companion, not just a draggable window, and meets the performance contract.

## Milestone 2 — Real Windows activity

- [ ] Windows media transport controls (WinRT)
- [ ] Supported system volume and output-device controls
- [ ] Battery, charging and power-setting events
- [ ] Permissioned notification provider with clear opt-in and source restrictions
- [ ] Explicit opt-in file transfer/download provider integrations
- [ ] Typed bounded event queue, event coalescing, rate limiting and stale-signal expiry

**Done when:** cards reflect real data and remain safe when providers fail or permission is denied.

## Milestone 3 — Personal workflows

- [ ] User-configurable rules and priority, app allow/deny lists
- [ ] Per-monitor preferences and global hotkeys
- [ ] Action palette and presets with consent for sensitive actions
- [ ] Optional secure clipboard utility with exclusion support
- [ ] Localization, light theme, high contrast and accessibility audit

## Milestone 4 — Extensions and stable release

- [ ] Versioned, manifest-driven sandboxed extension protocol
- [ ] Deny-by-default permissions; no in-process untrusted DLL execution
- [ ] Authenticity-verified update mechanism and rollback handling
- [ ] Signed binaries if possible, SHA256 checksums and dependency audits
- [ ] Manual and automated cross-device Windows 11 testing

## Explicitly out of scope

- Replacing/injecting into the Windows shell or games
- Password capture, hidden clipboard logging, telemetry
- Bypassing Defender, Smart App Control or UAC
- Replicating Apple's Dynamic Island
