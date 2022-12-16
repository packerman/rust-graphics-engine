use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug, Clone)]
pub struct SharedRef<T>(Rc<RefCell<T>>);

impl<T> SharedRef<T> {
    pub fn new(value: T) -> Self {
        Self {
            0: Rc::new(RefCell::new(value)),
        }
    }
}

impl<T> Deref for SharedRef<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
