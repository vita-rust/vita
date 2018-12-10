use core::time::Duration;

use psp2_sys::kernel::threadmgr::sceKernelDelayThread;
use psp2_sys::types::SceUInt;

/// Puts the current thread to sleep for at least the specified amount of time.
pub fn sleep(dur: Duration) {
    let micros = dur
        .as_secs()
        .saturating_mul(1000000)
        .saturating_add(dur.subsec_micros() as u64);
    unsafe {
        sceKernelDelayThread(micros as SceUInt);
    }
}
