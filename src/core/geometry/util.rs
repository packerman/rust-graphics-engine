use std::{iter, ops::Index};

pub fn select_by_indices<M, K, V, I>(indexed: &M, indices: I) -> Vec<V>
where
    M: Index<K, Output = V>,
    I: IntoIterator<Item = K>,
    V: Copy,
{
    indices.into_iter().map(|k| indexed[k]).collect()
}

pub fn replicate<T>(n: usize, t: T) -> impl Iterator<Item = T>
where
    T: Clone,
{
    iter::repeat(t).take(n)
}
