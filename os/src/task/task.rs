//! Types related to task management


use alloc::{vec::Vec};

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;


#[allow(unused)]
#[derive(Clone)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: Vec<u32>,
    pub time_duration: usize,
}

impl TaskInfo {
    pub fn new() -> TaskInfo{
        let mut syscall_times = Vec::new();
        syscall_times.resize(MAX_SYSCALL_NUM, 0);
        TaskInfo { status: TaskStatus::Running, syscall_times, time_duration: 0}
    }
}


/// The task control block (TCB) of a task.
#[derive(Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task info
    pub task_info: TaskInfo,
    ///
    pub time : usize
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
