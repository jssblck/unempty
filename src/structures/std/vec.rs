use std::ops::{Index, IndexMut};

/// `Vec` stores a single item in the data structure.
type Capacity = crate::Capacity<1>;

/// A non-empty vector of items.
///
/// The first entry is statically stored. Additional items are dynamically stored with
/// [`std::vec::Vec<T>`]; for memory and performance characteristics please review the documentation
/// for that module and type.
///
/// Does not currently support customizable allocators, nightly features, or unstable features.
/// If any of these are desired, please submit a PR for the parts you need!
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec<T> {
    first: T,
    dynamic: std::vec::Vec<T>,
}

impl<T> Vec<T> {
    /// Efficiently constructs a new instance with a single item.
    ///
    /// The underlying [`std::vec::Vec`] does not allocate unless more items are pushed.
    ///
    /// # Examples
    ///
    /// ```
    /// let vec: unempty::Vec<usize> = unempty::Vec::new(1);
    /// ```
    pub fn new(first: T) -> Self {
        Self {
            first,
            dynamic: Default::default(),
        }
    }

    /// Constructs a new instance with a single item and the specified capacity.
    ///
    /// Capacity is in two parts: the guaranteed portion of this data structure consumes 1 "capacity",
    /// and the dynamic portion of this data structure consumes the rest (the "additional capacity").
    ///
    /// "Additional capacity" follows the same rules as [`std::vec::Vec`]:
    ///
    /// The vector will be able to hold at least additional capacity elements without reallocating.
    /// This method is allowed to allocate for more elements than capacity.
    /// If additional capacity is 0, the vector will not allocate.
    ///
    /// It is important to note that although the returned vector has the minimum additional capacity specified,
    /// the vector will have a length of 1.
    /// For an explanation of the difference between length and capacity, see [`std::vec::Vec::with_capacity`].
    ///
    /// If it is imporant to know the exact allocated capacity, always use the `capacity` method after construction.
    ///
    /// When `T` is a zero-sized type, there will be no allocation and the additional capacity will always be `usize::MAX`.
    ///
    /// # Panics
    ///
    /// Panics if the additional capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    /// ```
    /// # use unempty::Capacity;
    /// let v = unempty::Vec::with_capacity("abc", Capacity::new_total(10));
    /// ```
    pub fn with_capacity(first: T, capacity: Capacity) -> Self {
        let dynamic = std::vec::Vec::with_capacity(capacity.dynamic());
        Self { first, dynamic }
    }

    /// Returns the number of elements the `Vec` can hold without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use unempty::Capacity;
    /// let cap = Capacity::new_total(10);
    /// let v = unempty::Vec::with_capacity("abc", cap);
    /// assert_eq!(v.capacity(), cap);
    /// ```
    pub fn capacity(&self) -> Capacity {
        Capacity::new_dynamic(self.dynamic.capacity())
    }

    /// Reserves capacity for at least additional more elements to be inserted in the given Vec<T>. The collection may reserve more space to speculatively avoid frequent reallocations. After calling reserve, capacity will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds isize::MAX bytes.
    ///
    /// # Examples
    /// ```
    /// # use unempty::Capacity;
    /// let mut v = unempty::Vec::new("abc");
    /// v.reserve(10);
    /// assert_eq!(v.capacity(), Capacity::new_additional(10));
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.dynamic.reserve(additional);
    }

    /// Reserves the minimum capacity for at least additional more elements to be inserted in the given Vec<T>. Unlike reserve, this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling reserve_exact, capacity will be greater than or equal to self.len() + additional. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer reserve if future insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds isize::MAX bytes.
    ///
    /// # Examples
    /// ```
    /// let mut vec = unempty::Vec::new("abc");
    /// vec.reserve_exact(10);
    /// assert!(vec.capacity().total() >= 11);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.dynamic.reserve_exact(additional);
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.first
        } else {
            &self.dynamic[index - 1]
        }
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.first
        } else {
            &mut self.dynamic[index - 1]
        }
    }
}
