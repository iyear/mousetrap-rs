/*!
Rust implementation of [https://github.com/inconshreveable/mousetrap](https://github.com/inconshreveable/mousetrap)

mousetrap is a tiny library that answers a single question.
On a Windows machine, was the process invoked by someone double-clicking on the executable file while browsing in explorer?

`mousetrap` provides a way to detect these invocations so that you can provide more helpful behavior and instructions on how to run the CLI tool.
*/
use std::process;

use windows_sys::Win32::Foundation;
use windows_sys::Win32::System::Diagnostics::ToolHelp;

#[cfg(not(windows))]
/// On non-Windows platforms, this function always returns false.
pub fn started_by_explorer() -> bool {
    false
}

#[cfg(windows)]
/// Returns true if the current process was started by Windows Explorer.
pub fn started_by_explorer() -> bool {
    // explorer.exe
    let explorer_exe: [Foundation::CHAR; 12] =
        [101, 120, 112, 108, 111, 114, 101, 114, 46, 101, 120, 101];

    let entry = match get_process_entry(process::id()) {
        None => return false,
        Some(e) => e,
    };

    return match get_process_entry(entry.th32ParentProcessID) {
        None => false,
        Some(e) => explorer_exe == e.szExeFile[0..12],
    };
}

fn get_process_entry(pid: u32) -> Option<ToolHelp::PROCESSENTRY32> {
    unsafe {
        let snapshot = ToolHelp::CreateToolhelp32Snapshot(ToolHelp::TH32CS_SNAPPROCESS, 0);
        if snapshot.eq(&0) {
            return None;
        }

        let entry = get_entry(snapshot, pid);
        Foundation::CloseHandle(snapshot);
        entry
    }
}

fn get_entry(snapshot: Foundation::HANDLE, pid: u32) -> Option<ToolHelp::PROCESSENTRY32> {
    unsafe {
        let mut entry: ToolHelp::PROCESSENTRY32 = ToolHelp::PROCESSENTRY32 {
            dwSize: std::mem::size_of::<ToolHelp::PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; 260],
        };
        if ToolHelp::Process32First(snapshot, &mut entry).eq(&0) {
            return None;
        }
        loop {
            if entry.th32ProcessID == pid {
                return Some(entry);
            }
            entry.szExeFile = [0; 260];
            if ToolHelp::Process32Next(snapshot, &mut entry).eq(&0) {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        // rust test is always not started by explorer
        assert_eq!(started_by_explorer(), false);
    }
}
