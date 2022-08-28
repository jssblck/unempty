use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

/// `Vec` stores a single item in the data structure.
type Capacity = crate::Capacity<1>;

/// Create an [`unempty::Vec`].
///
/// # Examples
/// ```
/// let v = unempty::vec![1, 2, 3];
/// let v = unempty::vec![1];
/// ```
#[macro_export]
macro_rules! vec {
    ($item:expr) => {{
        unempty::Vec::new($item)
    }};
    ($initial:expr, $( $additional:expr ),*) => {{
        let mut v = unempty::Vec::new($initial);
        $(
            v.push($additional);
        )*
        v
    }};
}

use crate::TryFromError;

/// A non-empty vector of items.
///
/// The first entry is statically stored. Additional items are dynamically stored with
/// [`std::vec::Vec<T>`]; for memory and performance characteristics please review the documentation
/// for that module and type.
///
/// # Completeness
///
/// `std::vec::Vec` has _many_ methods. These are being implemented as needed!
/// Please submit a PR or create an issue if you need a method!
///
/// # Unstable/nightly features
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

    /// Returns the number of elements in the data structure, also referred to as its ‘length’.
    /// Includes both static and dynamic portions of the data structure.
    ///
    /// # Examples
    /// ```
    /// let a = unempty::vec![1, 2, 3];
    /// assert_eq!(a.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.dynamic.len() + 1
    }

    /// Returns true if the vector contains no elements.
    /// This method _always_ returns `false`, because by defition an `unempty::Vec` cannot be empty.
    /// This method is included for API completeness and to make Clippy happy.
    ///
    /// # Examples
    /// ```
    /// let mut v = unempty::Vec::new("abcd");
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Removes the last element from a vector and returns it.
    ///
    /// If you’d like to pop the first element, consider using `VecDeque::pop_front` instead.
    ///
    /// # Consuming self
    ///
    /// Since this method may pop when there is only one item in the vector,
    /// it consumes the vector and optionally returns the vector with its new size.
    ///
    /// This is necessary because if the vector is popped when there is only one item,
    /// the "non-empty" guarantee of `unempty::Vec` is no longer possible.
    ///
    /// If this API is not desired, convert the instance to a `std::vec::Vec` and use that;
    /// `std::vec::Vec` has no such guarantee.
    ///
    /// # Examples
    /// ```
    /// let vec = unempty::vec![1, 2];
    ///
    /// let (vec, last) = vec.pop();
    /// assert_eq!(last, 2);
    /// assert_eq!(vec, Some(unempty::vec![1]));
    ///
    /// let (vec, last) = vec.expect("already tested").pop();
    /// assert_eq!(last, 1);
    /// assert_eq!(vec, None);
    /// ```
    pub fn pop(mut self) -> (Option<Self>, T) {
        if let Some(item) = self.dynamic.pop() {
            (Some(self), item)
        } else {
            (None, self.first)
        }
    }

    /// Appends an element to the back of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    /// ```
    /// let mut vec = unempty::vec![1, 2];
    /// vec.push(3);
    /// assert_eq!(vec, unempty::vec![1, 2, 3]);
    /// ```
    pub fn push(&mut self, item: T) {
        self.dynamic.push(item);
    }
}

impl<T> TryFrom<std::vec::Vec<T>> for Vec<T> {
    type Error = TryFromError;

    fn try_from(sv: std::vec::Vec<T>) -> Result<Self, Self::Error> {
        let mut sv = VecDeque::from(sv);
        if let Some(first) = sv.pop_front() {
            let mut v = Self::new(first);
            v.extend(sv.into_iter());
            Ok(v)
        } else {
            Err(TryFromError::SourceEmpty)
        }
    }
}

impl<T> TryFrom<VecDeque<T>> for Vec<T> {
    type Error = TryFromError;

    fn try_from(mut sv: VecDeque<T>) -> Result<Self, Self::Error> {
        if let Some(first) = sv.pop_front() {
            let mut v = Self::new(first);
            v.extend(sv.into_iter());
            Ok(v)
        } else {
            Err(TryFromError::SourceEmpty)
        }
    }
}

impl<T> From<Vec<T>> for std::vec::Vec<T> {
    fn from(sv: Vec<T>) -> Self {
        let mut v = std::vec::Vec::with_capacity(sv.len());
        v.push(sv.first);
        v.extend(sv.dynamic.into_iter());
        v
    }
}

impl<T> From<Vec<T>> for VecDeque<T> {
    fn from(sv: Vec<T>) -> Self {
        let mut v = VecDeque::with_capacity(sv.len());
        v.push_back(sv.first);
        v.extend(sv.dynamic.into_iter());
        v
    }
}

impl<T> Extend<T> for Vec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.dynamic.extend(iter);
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
