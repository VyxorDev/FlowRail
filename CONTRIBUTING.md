# Contributing to FlowRail

Thank you for improving an original, privacy-first Windows project.

1. For large behavior changes, open an issue describing the user benefit and privacy impact.
2. Keep application and UI logic in Rust; avoid JavaScript, Electron, browser/webview UI and required cloud services.
3. Use documented Windows APIs; do not inject into games, replace system components or use undocumented bypasses.
4. Add core-state tests and keep OS calls behind a platform adapter.
5. Before opening a PR, run: cargo fmt --all; cargo clippy --all-targets -- -D warnings; cargo test.
6. For UI changes, supply a screenshot or recording and document new permissions.

Be constructive and respectful. Report vulnerabilities privately according to SECURITY.md.
