# Performance and premium UX contract

This document is a product requirement, not a claim that v0.1 has already met the targets.

## Non-negotiable principles

1. **Idle means idle.** No busy loops, continuous repaint, high-frequency timers, or polling when a Windows event can be used.
2. **Power-aware rendering.** Prefer the lowest-power suitable GPU adapter. This is a preference request, not a guarantee that every driver selects the integrated GPU.
3. **No work for hidden UI.** When M1 adds tray-resident hide/minimize, stop UI repaint timers while hidden; event providers must suspend or reduce work where safe.
4. **Bounded work.** Every event queue, cache, history and provider buffer must have a limit, deduplication and expiry policy.
5. **No unnecessary effects.** No perpetual particles, animated backgrounds, video, large blurred backdrops or constant acrylic sampling.
6. **No background network.** No telemetry, polling servers, account sync, ads or automatic content downloads.
7. **Measure before optimizing.** Track CPU, private bytes, working set, GPU activity, startup time and wakeups on a documented Windows 11 baseline.

## Initial performance targets

Targets for a release candidate, measured on a documented Windows 11 x64 reference machine with integrated graphics. They are not current measurements or guarantees.

| Scenario | Initial target |
|---|---:|
| Visible idle CPU average, 5 minutes | <= 0.2% total system CPU |
| Hidden/tray idle CPU average, 5 minutes | <= 0.1% total system CPU |
| Active 1 Hz focus timer CPU average | <= 0.5% total system CPU |
| Idle private working set | <= 60 MiB |
| Cold start to interactive UI | <= 2 seconds |
| UI response to input | no perceptible delay; validate with frame/input traces |

If a target is missed, profile the cause and record the hardware/driver context. Do not game the measurement or silently lower the target.

## Rendering policy

- VSync for normal visible rendering.
- Repaint on input, OS/provider events, and actual animation frames.
- Timer display is whole-second precision: 1 Hz updates, not 4–60 Hz.
- While fully idle, schedule only a low-frequency refresh for time labels; M1 should suspend even that while hidden.
- Animate only state transitions, typically 120–180 ms, with a reduced-motion path.
- Avoid full-screen transparent surfaces and blur by default; optional native material effects must be user-disableable and tested for power impact.
- Use low-power GPU preference; verify actual adapter and GPU wake behavior on hybrid laptops.
- Cache formatted text and expensive derived state when measurements show they matter; keep UI render functions free of I/O.

## Visual quality bar

- Quiet, deliberate composition: strong hierarchy, generous spacing, restrained graphite surfaces, warm high-contrast text and one mint signal accent.
- Subtle borders and elevation; no excessive neon, gradients, glow, glass blur or ornamental animation.
- Hover, pressed, focused and disabled states must be coherent and responsive.
- Animations must be interruptible, deterministic and never block input.
- Consistent 8-point spacing rhythm, minimum 40–44 px interaction targets, keyboard navigation and visible focus.
- Support Windows scaling, high contrast, reduced motion, and screen readers.
- Never sacrifice readability or accessibility for visual minimalism.

## Repeatable manual measurement

Build release mode, launch FlowRail, then run:

    powershell -ExecutionPolicy Bypass -File .\scripts\measure-performance.ps1 -Seconds 300

Record OS build, CPU, GPU adapter, display refresh rate, FlowRail version, average CPU, private bytes and working set. Measure visible idle, focus timer, and (after M1) hidden/tray idle separately.

## Current implementation notes

- The focus timer currently schedules one repaint per second while running, and one per minute while idle. Input/window events repaint immediately.
- The app requests the low-power wgpu adapter to avoid preferring a discrete GPU for a simple 2D surface.
- The visual system is applied once at startup, not rewritten on every frame.
- These changes reduce avoidable work; actual CPU/RAM/GPU numbers still require Windows profiling.
