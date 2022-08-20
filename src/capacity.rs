use std::{
    cmp::{max, min},
    fmt::Display,
};

/// Defines the capacity for a data structure, considering the non-empty nature of data structures in this crate.
///
/// Many data structures provide a `with_capacity` or similar constructor to enable pre-allocation.
/// This has the potential to become confusing for users of a non-empty data structure:
/// _is this capacity the full capacity, or the additional capacity?_
///
/// To prevent this confusion, this crate uses [`Capacity`] for these types of methods.
///
/// # `N` constant
///
/// The `N` constant is the capacity size of the non-zero portion of the data structure.
/// For example, `unzero::Vec<T>` has a minimum size of 1, so its value for `N` is 1.
///
/// # Kinds of capacity
///
/// - `total`: Total capacity means "this is the total size of the data structure,
///   including the portion that is guaranteed by the non-empty data structure".
/// - `additional`: Additional capacity means "this is the size of the dynamic portion of the data structure,
///   treat the guaranteed portion separately"
///
/// For example, consider the following cases (`Vec` in the table below refers to [`unempty::Vec`]):
///
/// | Constructor                         | Total Capacity | Additional Capacity |
/// | ----------------------------------- | -------------- | ------------------- |
/// | `Vec::new(())`                      | 1              | 0                   |
/// | `Vec::with_capacity(10.into())`     | 10             | 9                   |
/// | `let v = Vec::new(()); v.push(());` | 2              | 1                   |
///
/// # `From` conversions
///
/// The `From` conversions provided for this data structure take the more conservative route and
/// treat the original value being converted from as _total_ capacity.
///
/// [Kinds of capacity]: #kinds-of-capacity
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Capacity<const N: usize> {
    total: usize,
    additional: usize,
}

impl<const N: usize> Capacity<N> {
    /// Create a [`Capacity`] with the provided _total_ capacity.
    /// If the provided total capacity is less than `N`, it is increased to `N`.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn new_total(capacity: usize) -> Self {
        let total = max(capacity, N);
        let additional = capacity - N;
        Self { total, additional }
    }

    /// Create a [`Capacity`] with the provided _additional_ capacity.
    /// If the provided additional capacity would cause an integer overflow when accounting for `N`,
    /// the additional capacity is reduced to `usize::MAX - N`.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn new_additional(capacity: usize) -> Self {
        let additional = min(capacity, usize::MAX - N);
        let total = additional + N;
        Self { total, additional }
    }

    /// Reference the _total_ capacity specified.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn total(&self) -> usize {
        self.total
    }

    /// Reference the _additional_ capacity specified.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn additional(&self) -> usize {
        self.additional
    }

    /// Reference the size of `N`.
    pub fn sizeof_n() -> usize {
        N
    }
}

impl<I, const N: usize> From<I> for Capacity<N>
where
    I: Into<usize>,
{
    fn from(total_capacity: I) -> Self {
        Self::new_total(total_capacity.into())
    }
}

impl<const N: usize> Default for Capacity<N> {
    fn default() -> Self {
        Self::new_total(N)
    }
}

impl<const N: usize> Display for Capacity<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "capacity(total: {}, additional: {})",
            self.total, self.additional
        )
    }
}
