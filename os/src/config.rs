// os/src/config.rs
//! JCore 中使用的常量定义

/// 用户态栈大小（以字节为单位）
/// 用户栈的默认大小设置为 2 个页面（每页 4096 字节）。
pub const USER_STACK_SIZE: usize = 4096 * 2;

/// 内核态栈大小（以字节为单位）
/// 内核栈的默认大小设置为 2 个页面（每页 4096 字节）。
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;

/// 最大应用程序数量
/// 系统中允许同时加载的最大应用程序数量为 4。
pub const MAX_APP_NUM: usize = 4;

/// 应用程序的起始基地址
/// 应用程序加载到内存的基地址为 0x80400000。
pub const APP_BASE_ADDRESS: usize = 0x80400000;

/// 应用程序大小限制（以字节为单位）
/// 单个应用程序的最大大小限制为 128 KB（0x20000 字节）。
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub use crate::board::CLOCK_FREQ;
