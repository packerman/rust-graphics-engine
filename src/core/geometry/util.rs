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

pub fn cycle_n<I, T>(into_iterator: I, n: usize) -> impl Iterator<Item = T>
where
    I: IntoIterator<Item = T> + Clone,
{
    iter::repeat(into_iterator).take(n).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_n_works() {
        let v = vec![1, 2, 3];
        assert_eq!(
            cycle_n(v, 4).collect::<Vec<_>>(),
            vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]
        )
    }
}
