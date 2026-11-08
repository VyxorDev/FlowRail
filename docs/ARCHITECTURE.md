# FlowRail — architecture and interaction design

## Product definition

FlowRail is a **signal track**: a narrow Windows desktop surface showing an intentionally short, prioritized vertical sequence of relevant activities and user-configured actions. It is **not** an Apple Dynamic Island, an app launcher disguised as a notch, a taskbar replacement, or an always-visible notification center.

## Design language

- **Anchor:** the screen edge on the user's chosen monitor; first release targets the right edge. Current v0.1 is a draggable borderless prototype, not yet docked.
- **Spine:** a thin highlighted vertical line unifies activity markers. Each marker corresponds to a panel with status, progress and actions.
- **Three densities (future):** collapsed (signal markers), glance (label and status), expanded (actions). Same location and input pattern in each state.
- **Motion:** ~120–180 ms fades/translations; a reduced-motion preference disables animation. No motion should obscure a fullscreen game.
- **Palette:** dark graphite canvas, muted slate containers, mint active signals, amber warnings. Color is never the only status indicator.
- **Accessibility:** screen-reader labels, keyboard focus, high contrast and reasonable hit areas are release criteria.
- **Monitor behavior:** respect working areas, per-monitor DPI, display changes and taskbar placement; future platform milestone.

## Target architecture

    FlowRail Shell (Rust / egui)
            |
            v
    ViewModel / reducer (typed snapshots)
            |
            v
    Core: activity registry, priority, focus, rules
            |
            v
    Bounded provider event queue + permission broker
            |
            v
    Windows providers via documented Win32 / WinRT APIs

The current single Rust binary is modular. After platform complexity justifies it, introduce a workspace:

- flowrail-core: domain types, scheduling, stable-ID deduplication, deterministic tests.
- flowrail-platform-windows: Win32/WinRT wrappers, audio, power, notifications and shell events.
- flowrail-shell: pure Rust egui renderer, keyboard behavior, native window layout.
- flowrail-app: lifecycle, single-instance guard, configuration, startup policy.
- flowrail-extensions: later, sandboxed external processes or WASM with explicit capabilities; no arbitrary in-process DLL loading.

Current modules:
- src/core/focus.rs: focus-state machine with pure tests, no OS calls.
- src/core/activity.rs: activity upsert, sorting and quiet suppression.
- src/platform.rs: Windows actions and GetTickCount64 binding.
- src/settings.rs: serde JSON local preferences with errors surfaced to the user.
- src/theme.rs: centralized visual tokens.
- src/app.rs: distinct signal-track layout and interaction controls.

## Native integrations — future providers

1. Media: WinRT GlobalSystemMediaTransportControlsSessionManager, session lifecycle, play/pause/skip.
2. Power: device power/battery events, avoid needless aggressive polling.
3. Focus: already implemented locally in Rust with no OS permissions.
4. Clipboard: opt-in and local only, exclude sensitive apps and unsupported fields; off by default.
5. Notifications: opt-in WinRT UserNotificationListener (permissions and OS version compatibility required).
6. Downloads: only browser extension or explicitly authorized source integration; do not pretend to detect arbitrary transfer progress.

A provider emits a normalized event with namespaced stable ID, urgency, declared actions, validity/expiry and optional progress. A bounded event broker coalesces duplicates, limits flooding, applies user rules and sends a snapshot to the UI thread. Providers cannot request arbitrary system access merely by publishing an event.

## Trust model

- Standard-user application; never require UAC elevation, disabling antivirus, or bypassing Windows protections.
- No user login, required internet, telemetry, ad/tracker SDK, keyboard capture or unrequested screen recording.
- Never read foreground window titles by default.
- Explicit permissions for integration features, and a plainly indicated unavailable state when access is denied.
- No silently downloaded or executed third-party native extensions.
- CI build provenance, dependency review, checksums and code signing are release deliverables, not capabilities of v0.1.

## Performance and UX acceptance gates

These are **targets, not measurements**: low idle CPU via event-driven updates, low memory versus browser-based shells, correct per-monitor DPI, no interference with exclusive fullscreen, robust wake/sleep and explorer.exe restart, useful keyboard-only navigation, high contrast, reduced motion and clear quit semantics.

## Why egui rather than raw Win32 UI?

egui/eframe offers a Rust-only programmable UI and Windows desktop viewports, enabling quick iterations. Raw Win32+Direct2D is an alternative if measured shell performance, accessibility or overlay requirements demand it. A borderless window by itself is *not* a complete native system overlay; M1 will implement tray, anchoring and fullscreen policy.
