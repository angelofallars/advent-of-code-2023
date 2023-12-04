use std::iter::once;

#[inline]
pub fn is_end<T>(input: &Vec<T>, pos: usize) -> bool {
    input.len() <= pos
}

/// append works exactly like Go's `append` function.
#[inline]
pub fn append<T: IntoIterator<Item = U> + FromIterator<U>, U>(i: T, elem: U) -> T {
    i.into_iter().chain(once(elem)).collect()
}

#[inline]
pub fn extend<
    T: IntoIterator<Item = V> + FromIterator<V>,
    U: IntoIterator<Item = V> + FromIterator<V>,
    V,
>(
    a: T,
    b: U,
) -> U {
    a.into_iter().chain(b).collect()
}

#[inline]
pub fn advance(i: usize) -> usize {
    i + 1
}
