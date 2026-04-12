//! Process management syscalls
use crate::{
    config::{APP_SIZE_LIMIT, USER_STACK_SIZE},
    loader::{get_app_base, get_user_sp},
    task::{
        current_syscall_times, current_task_id, exit_current_and_run_next,
        suspend_current_and_run_next,
    },
    timer::get_time_ms,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[repr(usize)]
enum TraceRequest {
    Read = 0,
    Write = 1,
    Syscall = 2,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ms = get_time_ms();
    unsafe {
        *ts = TimeVal {
            sec: ms / 1000,
            usec: (ms % 1000) * 1000,
        };
    }
    0
}

pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        x if x == TraceRequest::Read as usize => trace_read(id as *const u8),
        x if x == TraceRequest::Write as usize => trace_write(id as *mut u8, data as u8),
        x if x == TraceRequest::Syscall as usize => current_syscall_times(id),
        _ => -1,
    }
}

fn trace_read(addr: *const u8) -> isize {
    if !is_trace_addr_valid(addr as usize) {
        return -1;
    }
    unsafe { addr.read_volatile() as isize }
}

fn trace_write(addr: *mut u8, data: u8) -> isize {
    if !is_trace_addr_valid(addr as usize) {
        return -1;
    }
    unsafe {
        addr.write_volatile(data);
    }
    0
}

fn is_trace_addr_valid(addr: usize) -> bool {
    let app_id = current_task_id();
    let app_base = get_app_base(app_id);
    let app_end = app_base + APP_SIZE_LIMIT;
    let user_sp = get_user_sp(app_id);
    let user_stack_bottom = user_sp - USER_STACK_SIZE;
    (app_base..app_end).contains(&addr) || (user_stack_bottom..user_sp).contains(&addr)
}
