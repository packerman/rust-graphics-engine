use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Cached<T>(RefCell<Option<T>>);

impl<T> Cached<T> {
    pub fn new() -> Self {
        Self(RefCell::new(None))
    }

    pub fn clear(&self) -> bool {
        self.0.borrow_mut().take().is_some()
    }

    pub fn with_cached_ref<C, F, R>(&self, if_absent: C, action: F) -> R
    where
        C: FnOnce() -> T,
        F: FnOnce(&T) -> R,
    {
        action(self.0.borrow_mut().get_or_insert_with(if_absent))
    }
}

impl<T: Copy> Cached<T> {
    pub fn get<F>(&self, if_absent: F) -> T
    where
        F: FnOnce() -> T,
    {
        *self.0.borrow_mut().get_or_insert_with(if_absent)
    }
}
