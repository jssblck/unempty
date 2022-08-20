//! Non-empty data structures for Rust.
//!
//! Other crates are more focused for a single data structure (e.g. Vec or HashMap).
//! The goal of this crate is to contain non-empty implementations for all commonly used data structures.
//!
//! Of course, "all" is quite a lot. I intend to add to this as I need non-empty data structures.
//! If you need one, PRs are extremely welcome!
//!
//! # Path based imports
//!
//! Data structures in this crate use the same name as the data structure they're wrapping.
//! It is highly recommended to use the full path name of the data structures instead of shadowing others.
//!
//! For example, instead of:
//! ```
//! use unempty::Vec;
//! let v = Vec::new(());
//! ```
//!
//! Please **strongly** consider:
//! ```
//! let v = unempty::Vec::new(());
//! ```
//!
//! This reduces confusion for people reading your source code later,
//! _and_ improves the ability for your program to interact with both non-empty and standard data structures.
//!
//! # Why
//!
//! Expressing constraints in the type system is powerful, and non-empty data structures are no exception.
//!
//! A data structure that is impossible to be empty frees developers from needing to check for the empty case,
//! and allows them to express an expectation in their types rather than solely at runtime or in their documentation.
//!
//! This also enables some convenience features or even performance improvements.
//! For example, an `unempty::Vec` can always fulfill a call to `first` or `last`, so these don't need to be `Option`.
//!
//! # Esoteric support
//!
//! ## Unsafe
//!
//! Generally, this library doesn't re-implement the unsafe portions of the underlying data structure.
//! This is mostly because I haven't needed to use them. If you need them, please open a PR!
//!
//! ## Nightly or Beta
//!
//! Generally, this library doesn't re-implement the nightly or beta portions of the underlying data structure.
//! This is mostly because I haven't needed to use them. If you need them, please open a PR!

#![deny(clippy::unwrap_used)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(not(any(feature = "std")))]
compile_error!("The `std` feature is currently required. Adding support for `no-std` is backwards compatible! If you need this, a PR is extremely welcome!");

mod capacity;
mod structures;

pub use capacity::*;
pub use structures::*;
