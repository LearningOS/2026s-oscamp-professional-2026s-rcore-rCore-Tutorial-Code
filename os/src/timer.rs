//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::UPSafeCell;
use lazy_static::*;
use riscv::register::time;
/// The number of ticks per second
const TICKS_PER_SEC: usize = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: usize = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: usize = 1_000_000;

lazy_static! {
    static ref SOFTWARE_TIME_MS: UPSafeCell<usize> = unsafe { UPSafeCell::new(1) };
}

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    let ticks = time::read();
    let hardware_ms = if ticks == 0 {
        0
    } else {
        (ticks * MSEC_PER_SEC).div_ceil(CLOCK_FREQ)
    };
    let software_ms = *SOFTWARE_TIME_MS.exclusive_access();
    hardware_ms.max(software_ms)
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    time::read() * MICRO_PER_SEC / CLOCK_FREQ
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/// Advance the software time source by the given milliseconds.
pub fn advance_time_ms(delta_ms: usize) {
    let mut software_ms = SOFTWARE_TIME_MS.exclusive_access();
    *software_ms += delta_ms;
}
