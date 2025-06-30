#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::BPF_ANY,
    helpers::bpf_get_current_pid_tgid,
    macros::{kprobe, map},
    maps::{PerfEventArray, HashMap},
    programs::ProbeContext,
    EbpfContext,
};
use aya_log_ebpf::info;
use fw_common::{FileEvent, MAX_PATH_LEN, MAX_FILENAME_LEN};

/// PerfEvent array for sending events to userspace
#[map]
static EVENTS: PerfEventArray<FileEvent> = PerfEventArray::new(0);

/// Map to track opened files by file descriptor
#[map]
static OPEN_FILES: HashMap<u64, FileEvent> = HashMap::new(1024);

/// Kernel probe for openat system call
#[kprobe]
pub fn openat(ctx: ProbeContext) -> u32 {
    match try_openat(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_openat(ctx: ProbeContext) -> Result<u32, u32> {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;
    let tgid = pid_tgid as u32;

    // Get the filename parameter (second argument to openat)
    let filename_ptr: *const u8 = ctx.arg(1).ok_or(1u32)?;

    let mut event = FileEvent {
        pid,
        tgid,
        path: [0u8; MAX_PATH_LEN],
        filename: [0u8; MAX_FILENAME_LEN],
        event_type: 0, // 0 = open
    };

    // Safely read the filename from userspace
    let ret = unsafe {
        bpf_probe_read_user_str(
            event.path.as_mut_ptr(),
            MAX_PATH_LEN as u32,
            filename_ptr as *const core::ffi::c_void,
        )
    };

    if ret < 0 {
        return Err(1);
    }

    // Extract just the filename from the full path
    extract_filename(&event.path, &mut event.filename);

    // Store the event temporarily with a key based on current context
    // We'll use this in the return probe to get the file descriptor
    let key = pid_tgid;
    OPEN_FILES.insert(&key, &event, BPF_ANY as u64).map_err(|_| 1u32)?;

    info!(&ctx, "File open: pid={} path={:?}", pid, event.path);
    Ok(0)
}

/// Kernel return probe for openat system call
#[kprobe(name = "openat_ret")]
pub fn openat_ret(ctx: ProbeContext) -> u32 {
    match try_openat_ret(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_openat_ret(ctx: ProbeContext) -> Result<u32, u32> {
    let pid_tgid = bpf_get_current_pid_tgid();
    let ret_value: i64 = ctx.ret().ok_or(1u32)?;

    // Only process successful opens (positive file descriptor)
    if ret_value < 0 {
        OPEN_FILES.remove(&pid_tgid).ok();
        return Ok(0);
    }

    // Get the stored event from the open call
    let event = OPEN_FILES.get(&pid_tgid).ok_or(1u32)?;
    let mut event = *event;

    // Clean up the temporary storage
    OPEN_FILES.remove(&pid_tgid).ok();

    // Send the event to userspace
    EVENTS.output(&ctx, &event, 0);

    info!(&ctx, "File opened successfully: fd={} pid={}", ret_value, event.pid);
    Ok(0)
}

/// Kernel probe for close system call
#[kprobe]
pub fn close(ctx: ProbeContext) -> u32 {
    match try_close(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_close(ctx: ProbeContext) -> Result<u32, u32> {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;
    let tgid = pid_tgid as u32;

    // Get the file descriptor parameter
    let fd: i32 = ctx.arg(0).ok_or(1u32)?;

    // We can't easily get the filename from just the fd in eBPF,
    // so we'll send a close event with the fd and let userspace
    // correlate it with previously opened files
    let event = FileEvent {
        pid,
        tgid,
        path: [0u8; MAX_PATH_LEN], // Will be filled by userspace
        filename: [0u8; MAX_FILENAME_LEN], // Will be filled by userspace
        event_type: 1, // 1 = close
    };

    // For close events, we store the fd in the first 4 bytes of path
    unsafe {
        core::ptr::write(event.path.as_ptr() as *mut i32, fd);
    }

    EVENTS.output(&ctx, &event, 0);
    info!(&ctx, "File close: pid={} fd={}", pid, fd);
    Ok(0)
}

/// Extract filename from a full path
fn extract_filename(path: &[u8; MAX_PATH_LEN], filename: &mut [u8; MAX_FILENAME_LEN]) {
    let mut last_slash = 0;

    // Find the last slash in the path
    for (i, &byte) in path.iter().enumerate() {
        if byte == 0 {
            break;
        }
        if byte == b'/' {
            last_slash = i + 1;
        }
    }

    // Copy from last slash to end (or from beginning if no slash)
    let mut copied = 0;
    for i in last_slash..MAX_PATH_LEN {
        if copied >= MAX_FILENAME_LEN - 1 || path[i] == 0 {
            break;
        }
        filename[copied] = path[i];
        copied += 1;
    }
    // Ensure null termination
    if copied < MAX_FILENAME_LEN {
        filename[copied] = 0;
    }
}

/// Helper function to read user string (declaration)
extern "C" {
    fn bpf_probe_read_user_str(
        dst: *mut u8,
        size: u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
