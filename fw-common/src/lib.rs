//! Shared definitions between eBPF program and userspace application

/// Maximum path length we can capture
pub const MAX_PATH_LEN: usize = 256;

/// Maximum filename length
pub const MAX_FILENAME_LEN: usize = 64;

/// Event data structure sent from eBPF program to userspace
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FileEvent {
    /// Process ID that triggered the event
    pub pid: u32,
    /// Thread group ID
    pub tgid: u32,
    /// File path (null-terminated)
    pub path: [u8; MAX_PATH_LEN],
    /// Filename only (null-terminated)
    pub filename: [u8; MAX_FILENAME_LEN],
    /// Event type: 0=open, 1=close
    pub event_type: u32,
}

impl FileEvent {
    /// Get the path as a string
    pub fn path_str(&self) -> Result<&str, std::str::Utf8Error> {
        let null_pos = self.path.iter().position(|&b| b == 0).unwrap_or(self.path.len());
        std::str::from_utf8(&self.path[..null_pos])
    }

    /// Get the filename as a string
    pub fn filename_str(&self) -> Result<&str, std::str::Utf8Error> {
        let null_pos = self.filename.iter().position(|&b| b == 0).unwrap_or(self.filename.len());
        std::str::from_utf8(&self.filename[..null_pos])
    }

    /// Check if this is an open event
    pub fn is_open(&self) -> bool {
        self.event_type == 0
    }

    /// Check if this is a close event
    pub fn is_close(&self) -> bool {
        self.event_type == 1
    }
}
