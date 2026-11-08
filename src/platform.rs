//! Small platform boundary. All Windows-only system calls stay here.
use std::{io, process::Command, time::Duration};

#[derive(Clone, Copy)]
pub enum QuickAction {
    Explorer,
    Settings,
    TaskManager,
}

pub fn launch(action: QuickAction) -> io::Result<()> {
    let mut command = match action {
        QuickAction::Explorer => Command::new("explorer.exe"),
        QuickAction::Settings => {
            let mut command = Command::new("explorer.exe");
            command.arg("ms-settings:");
            command
        }
        QuickAction::TaskManager => Command::new("taskmgr.exe"),
    };
    command.spawn().map(|_| ())
}

#[cfg(target_os = "windows")]
pub fn system_uptime() -> Option<Duration> {
    // SAFETY: GetTickCount64 has no arguments or pointers and returns a
    // monotonic millisecond counter maintained by Windows.
    let milliseconds = unsafe {
        windows_sys::Win32::System::SystemInformation::GetTickCount64()
    };
    Some(Duration::from_millis(milliseconds))
}

#[cfg(not(target_os = "windows"))]
pub fn system_uptime() -> Option<Duration> {
    None
}
