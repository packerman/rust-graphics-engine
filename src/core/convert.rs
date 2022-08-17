use anyhow::Result;

pub trait FromWithContext<C, T>: Sized {
    fn from(context: &C, value: T) -> Result<Self>;
}

pub trait IntoWithContext<C, T> {
    fn into(self, context: &C) -> Result<T>;
}

impl<C, T, U> IntoWithContext<C, U> for T
where
    U: FromWithContext<C, T>,
{
    fn into(self, context: &C) -> Result<U> {
        U::from(context, self)
    }
}
