/// A non-empty hashmap of items.
///
/// The first key-value pair is statically stored. Additional pairs are dynamically stored with
/// [`std::collections::HashMap<K, V>`]; for memory and performance characteristics please
/// review the documentation for that module and type.
///
/// # Completeness
///
/// `std::collections::HashMap` has _many_ methods. These are being implemented as needed.
/// Please submit a PR or create an issue if you need a method!
///
/// # Unstable/nightly features
///
/// Does not currently support customizable allocators, nightly features, or unstable features.
/// If any of these are desired, please submit a PR for the parts you need!
#[derive(Clone, Debug)]
pub struct HashMap<K, V> {
    first: (K, V),
    dynamic: std::collections::HashMap<K, V>,
}

/// This structure stores a single item statically.
type Capacity = crate::Capacity<1>;
