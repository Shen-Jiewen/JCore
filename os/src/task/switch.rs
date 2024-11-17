// os/src/task/switch.rs
//! `__switch` 的 Rust 包装器。
//!
//! 在这里切换到不同任务的上下文。实际的实现不能使用 Rust，
//! 并且（本质上）必须使用汇编语言。（你知道为什么吗？）
//! 因此，该模块实际上只是对 `switch.S` 的一个包装。

use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    /// Switch to the context of `next_task_cx_ptr`, saving the current context.
    /// in `current_task_cx_ptr`.
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}