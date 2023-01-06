use std::{fmt::Debug, ops::Rem};

use anyhow::{Error, Result};
use num_traits::Zero;

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

pub fn contains<T, E>(value: &T, array: &[T], error: E) -> Result<()>
where
    T: PartialEq + Debug,
    E: Fn(&T) -> Error,
{
    self::assert(array.contains(value), || error(value))
}

pub fn not_empty<T, E>(slice: &[T], error: E) -> Result<()>
where
    T: Debug,
    E: Fn() -> Error,
{
    self::assert(!slice.is_empty(), error)
}

pub fn equal_on<T, F, K, E>(x: &T, y: &T, f: F, error: E) -> Result<()>
where
    F: Fn(&T) -> K,
    K: PartialEq,
    E: Fn() -> Error,
{
    self::assert(f(x) == f(y), error)
}

pub fn divisible_by<N, E>(n: N, k: N, error: E) -> Result<()>
where
    N: Debug + Rem<Output = N> + Zero,
    E: Fn() -> Error,
{
    self::assert((n % k).is_zero(), error)
}
