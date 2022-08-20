# unempty

Non-empty data structures for Rust.

Other crates are more focused for a single data structure (e.g. `Vec` or `HashMap`).
The goal of this crate is to contain non-empty implementations for all commonly used data structures.

Of course, "all" is quite a lot. I intend to add to this as I need non-empty data structures.
If you need one, PRs are extremely welcome!

# why

Expressing constraints in the type system is powerful, and non-empty data structures are no exception.

A data structure that is impossible to be empty frees developers from needing to check for the empty case,
and allows them to express an expectation in their types rather than solely at runtime or in their documentation.

This also enables some convenience features or even performance improvements.
For example, an `unempty::Vec` can always fulfill a call to `first` or `last`, so these don't need to be `Option`.

# inspirations

- https://lib.rs/crates/nonempty
- https://hackage.haskell.org/package/base-4.17.0.0/docs/Data-List-NonEmpty.html
