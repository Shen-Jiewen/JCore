// os/src/task/mod.rs

//! 任务管理实现模块
//! 
//! 这个模块实现了任务管理的所有内容,例如启动和切换任务
//! 在操作系统中,有一个全局的 [`TaskManager`] 实例 `TASK_MANAGER` 控制所有任务.
//! 
//! 在 `switch.rs` 文件中存在一个 `__switch` 汇编函数, 需要注意在该函数周围的控制流.
//! 可能不会如预期般直观.

pub use context::TaskContext;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sync::UPSafeCell;
use crate::sbi::shutdown;

mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;


/// 任务管理器,管理所有的任务
/// 
/// `TaskManager` 实现了所有任务状态转换和任务上下切换的相关函数
/// 在模块层可以找到一些结构体的包装函数.
/// 
/// `TaskManager` 中大部分功能通过 `inner` 字段进行隐藏,通过运行时延迟借用检查.
/// `TaskManager` 内部的函数演示了如何使用 `inner` 字段.
pub struct TaskManager {
    /// 任务数量
    num_app: usize,
    /// 使用 inner 字段来获得可变访问权限
    inner: UPSafeCell<TaskManagerInner>,
}

/// 任务管理内部结构
pub struct TaskManagerInner {
    /// 任务列表
    tasks: [TaskControlBlock; MAX_APP_NUM], // 存放所有任务的任务块
    /// 当前正在运行的任务ID
    current_task: usize,
}

lazy_static! {
    /// 全局变量: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();    // 获取当前系统中应用程序数量
        let mut tasks = [TaskControlBlock{
            task_cx: TaskContext::zero_init(),  // 初始化任务上下文为0
            task_status: TaskStatus::UnInit,    // 设置初始状态为未初始化
        }; MAX_APP_NUM];    //初始化任务数组
        for (i, task) in tasks.iter_mut().enumerate() {
            // 初始化任务上下文
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            // 设置任务状态为就绪
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,    //任务总数
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner{
                    tasks,              // 初始化任务列表
                    current_task: 0,    // 初始化当前任务索引
                })
            },
        }
    };
}

impl TaskManager {
    /// 运行任务列表的第一个任务
    ///
    /// 通常任务列表的第一个任务是空闲任务(在后续版本中称为零进程)
    fn run_first_task(&self) -> ! {
        // 获取任务管理结构体的使用权
        let mut inner = self.inner.exclusive_access();
        // 获取第一个任务的控制块
        let task0 = &mut inner.tasks[0];
        // 设置状态为运行中
        task0.task_status = TaskStatus::Running;
        // 获取下一个任务的上下文指针
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        // 释放inner, 确保 exclusive_access 锁的释放
        drop(inner);
        // 初始化一个临时上下文
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!")
    }

    /// 将当前任务标记为挂起
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// 将当前运行任务的状态标记为退出
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
    
    /// 查找下一个需要运行的任务并返回其ID
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// 切换到当前"运行中"任务到找到的下一个任务
    fn run_next_task(&self) {
            if let Some(next) = self.find_next_task() {
                let mut inner = self.inner.exclusive_access();
                let current = inner.current_task;
                // 切换到下一个任务
                inner.tasks[next].task_status = TaskStatus::Running;
                inner.current_task = next;
                // 获取当前任务和下一个任务的任务上下文指针
                let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
                let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
                // 释放任务管理结构体使用权
                drop(inner);
                // 在此之前,应该手动释放所有必须释放的局部变量
                unsafe {
                    // 切换到下一个任务
                    __switch(current_task_cx_ptr, next_task_cx_ptr);
                }
            } else {
                // 打印任务完成消息
                println!("All applications completed!");
                shutdown(false);
            }
    }
}


/// 运行第一个任务
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// 切换到下一个任务
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// 暂停当前任务
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// 退出当前任务
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// 暂停当前任务并且切换到下一个任务
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// 退出当前任务并且切换到下一个任务
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

