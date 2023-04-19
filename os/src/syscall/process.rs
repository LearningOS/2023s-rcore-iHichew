//! Process management syscalls
use alloc::vec::Vec;

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER, self},
    timer::get_time_us,
};

///
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
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
    fn new() -> TaskInfo{
        TaskInfo { status: TaskStatus::Running, syscall_times: [0; MAX_SYSCALL_NUM], time: 0 }
    }
}

impl TryFrom<task::task::TaskInfo> for TaskInfo {
    type Error = Vec<u32>;

    fn try_from(other: task::task::TaskInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            status: other.status,
            syscall_times: other.syscall_times.try_into()?,
            time: other.time_duration,
        })
    }
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
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    
    let t = 
        unsafe{
        ((*ts).sec & 0xffff) * 1000 + (*ts).usec / 1000
    };
    
    TASK_MANAGER.modify_time(t);
    0
}


/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe {
        *_ti = TASK_MANAGER.get_info().try_into().unwrap();
    }
    0
}
