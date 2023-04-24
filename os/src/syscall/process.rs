//! Process management syscalls
use alloc::vec::Vec;

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, TASK_MANAGER,
    }, mm::{page_table::PageTable, VirtAddr}, timer::get_time_us,
};


#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

impl TaskInfo {
    pub fn new() -> TaskInfo{
        let mut syscall_times = [0; MAX_SYSCALL_NUM];
        //syscall_times.resize(MAX_SYSCALL_NUM, 0);
        TaskInfo { status: TaskStatus::Running, syscall_times, time: 0}
    }
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = current_user_token();
    let page_table = PageTable::from_token(token);
    let addr = page_table.translate_va(VirtAddr::from(_ts as usize)).unwrap();
    let us = get_time_us();
    let ts = addr.0 as  *mut TimeVal;
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
        
    };
    let t = 
        unsafe{
        ((*ts).sec & 0xffff) * 1000 + (*ts).usec / 1000
    };

    TASK_MANAGER.modify_time(t);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let token = current_user_token();
    let page_table = PageTable::from_token(token);
    let addr = page_table.translate_va(VirtAddr::from(_ti as usize)).unwrap();
    let ti = addr.0 as *mut TaskInfo;
    unsafe {
        *ti = TASK_MANAGER.get_info();
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    TASK_MANAGER.mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    TASK_MANAGER.munmap(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
