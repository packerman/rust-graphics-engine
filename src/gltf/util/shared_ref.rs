use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone, Default)]
pub struct SharedRef<T>(Rc<RefCell<T>>);

impl<T> SharedRef<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    pub fn new_cyclic<F>(data_fn: F) -> SharedRef<T>
    where
        F: FnOnce(&WeakRef<T>) -> T,
    {
        Self(Rc::new_cyclic(|weak: &WeakRef<T>| {
            RefCell::new(data_fn(weak))
        }))
    }

    pub fn upgrade(weak: &WeakRef<T>) -> Option<SharedRef<T>> {
        weak.upgrade().map(Self)
    }
}

impl<T> Deref for SharedRef<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type WeakRef<T> = Weak<RefCell<T>>;
