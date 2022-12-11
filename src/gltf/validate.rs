use std::fmt::Debug;

use anyhow::{Error, Result};

pub fn assert<E>(condition: bool, error: E) -> Result<()>
where
    E: Fn() -> Error,
{
    if condition {
        Ok(())
    } else {
        Err(error())
    }
}

pub fn optional<T, F>(value: &Option<T>, condition: F) -> Result<()>
where
    F: Fn(&T) -> Result<()>,
{
    if let Some(value) = value {
        condition(value)
    } else {
        Ok(())
    }
}

pub fn contains<T, F>(value: &T, array: &[T], error: F) -> Result<()>
where
    T: PartialEq + Debug,
    F: Fn(&T) -> Error,
{
    self::assert(array.contains(value), || error(value))
}
