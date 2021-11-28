//! [`get_top()`] implements a brute force method of selecting the
//! largest `top` integers from the given collection.
//!
//! [`get_top()`]: fn.get_top.html
//!
//! ```
//! # use topqueue::page1::*;
//! let n_rands: Vec<i32> = make_rands(10).collect();
//! let top_m = get_top(&n_rands, 3);
//!
//! assert_eq!(
//!     n_rands.iter().max(),
//!     top_m.iter().max()
//! );
//! ```
//!
//! Same as above, but MOAR.
//!
//! ```
//! # use topqueue::page1::*;
//! let n_rands: Vec<i32> = make_rands(1_000_000).collect();
//! let top_m = get_top(&n_rands, 100);
//!
//! assert_eq!(
//!     n_rands.iter().max(),
//!     top_m.iter().max()
//! );
//! ```
//!
//! Same method on a rediciously inefficient scale!
//!
//! ```no_run
//! # use topqueue::page1::*;
//! let n_rands: Vec<i32> = make_rands(100_000_000).collect();
//! let top_m = get_top(&n_rands, 100);
//!
//! assert_eq!(
//!     n_rands.iter().max(),
//!     top_m.iter().max()
//! );
//! ```
//!
//! This file correspond to example code in the first [blog post].
//!
//! [blog post]: https://lex.penryu.dev/posts/scala-interview-1

use std::iter::from_fn;
use rand::{Rng, thread_rng};

/// Creates a new iterator of random numbers of the given size.
pub fn make_rands(len: usize) -> impl Iterator<Item=i32> {
    from_fn(move || Some(thread_rng().gen())).take(len)
}

/// Takes a vector `vec` and returns the `top` largest values.
pub fn get_top(vec: &Vec<i32>, top: usize) -> Vec<i32> {
    let mut dupe = vec.clone().into_iter().collect::<Vec<i32>>();
    dupe.sort_unstable_by(|a, b| b.cmp(a));
    dupe.into_iter().take(top).collect()
}
