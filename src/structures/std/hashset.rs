/// A non-empty hashset of items.
///
/// The first entry is statically stored. Additional items are dynamically stored with
/// [`std::collections::HashSet<T>`]; for memory and performance characteristics please review the documentation
/// for that module and type.
///
/// # Completeness
///
/// `std::collections::HashSet` has _many_ methods. These are being implemented as needed.
/// Please submit a PR or create an issue if you need a method!
///
/// # Unstable/nightly features
///
/// Does not currently support customizable allocators, nightly features, or unstable features.
/// If any of these are desired, please submit a PR for the parts you need!
#[derive(Clone, Debug)]
pub struct HashSet<T> {
    first: T,
    dynamic: std::collections::HashSet<T>,
}

/// This structure stores a single item statically.
type Capacity = crate::Capacity<1>;
