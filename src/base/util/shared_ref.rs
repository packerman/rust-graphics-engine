use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type SharedRef<T> = Rc<RefCell<T>>;

pub type WeakRef<T> = Weak<RefCell<T>>;

pub fn strong<T>(value: T) -> SharedRef<T> {
    Rc::new(RefCell::new(value))
}

pub fn cyclic<F, T>(data_fn: F) -> SharedRef<T>
where
    F: FnOnce(&WeakRef<T>) -> T,
{
    Rc::new_cyclic(|weak: &WeakRef<T>| RefCell::new(data_fn(weak)))
}

pub fn weak<T>() -> WeakRef<T> {
    Weak::new()
}
