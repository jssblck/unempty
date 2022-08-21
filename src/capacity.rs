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
/// The `N` constant is the capacity size of the statically sized portion of the data structure.
/// For example, `unzero::Vec<T>` statically stores one `T`, so its value for `N` is 1.
///
/// # Kinds of capacity
///
/// - `total`: Total capacity means "this is the total size of the data structure,
///   including the statically sized portion maintained by the non-empty data structure".
/// - `dynamic`: Dynamic capacity means "this is the size of the dynamic portion of the data structure".
///   Most non-empty data structures are backed by some other dynamically growable structure,
///   this size represents the size of that structure directly.
///
/// For example, consider the following cases (`Vec` in the table below refers to [`unempty::Vec`]):
///
/// | Constructor                         | Total Capacity | Dynamic Capacity |
/// | ----------------------------------- | -------------- | ---------------- |
/// | `Vec::new(())`                      | 1              | 0                |
/// | `Vec::with_capacity(10.into())`     | 10             | 9                |
/// | `let v = Vec::new(()); v.push(());` | 2              | 1                |
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
    dynamic: usize,
}

impl<const N: usize> Capacity<N> {
    /// Create a [`Capacity`] with the provided _total_ capacity.
    /// If the provided total capacity is less than `N`, it is increased to `N`.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn new_total(capacity: usize) -> Self {
        let total = max(capacity, N);
        let dynamic = capacity - N;
        Self { total, dynamic }
    }

    /// Create a [`Capacity`] with the provided capacity for the dynamic portion of the data structure.
    /// If the provided dynamic capacity would cause an integer overflow when accounting for `N`,
    /// the dynamic capacity is reduced to `usize::MAX - N`.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn new_dynamic(capacity: usize) -> Self {
        let dynamic = min(capacity, usize::MAX - N);
        let total = dynamic + N;
        Self { total, dynamic }
    }

    /// Reference the _total_ capacity specified.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn total(&self) -> usize {
        self.total
    }

    /// Reference the _dynamic_ capacity specified.
    ///
    /// For definitions on kinds of capacity, see *[Kinds of capacity]*.
    pub fn dynamic(&self) -> usize {
        self.dynamic
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
            "capacity(static_size: {}, dynamic_size: {})",
            self.total, self.dynamic
        )
    }
}
