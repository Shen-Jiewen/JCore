// os/src/sync/up.rs

use core::cell::{RefCell, RefMut};

/// UPSafeCell 是一个用于单处理器环境的线程不安全封装类型。
pub struct UPSafeCell<T> {
    // 内部数据，通过 RefCell 封装
    inner: RefCell<T>,
}

/// 实现 Sync 特性，允许在多线程环境中安全访问。
/// 使用者需保证在单处理器环境下使用 UPSafeCell。
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// 用户需要保证UPSafeCell只能在单处理器的环境中使用
    /// # Safety
    /// 这个方法是'unsafe'的。因为如果在多处理器的环境中使用,可能会导致数据竞争。
    pub unsafe fn new(value: T) -> Self {
        Self{ inner: RefCell::new(value)}
    }

    /// 获取内部数据的独占访问权限,如果数据已经被借用,则会触发panic!
    pub fn exclusive_access(&self) -> RefMut<'_,T> {
        self.inner.borrow_mut()
    }
}