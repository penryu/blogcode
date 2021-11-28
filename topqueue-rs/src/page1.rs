//! [`get_top()`] implements a brute force method of selecting the
//! largest `top` integers from the given collection.
//!
//! [`get_top()`]: fn.get_top.html
//!
//! ```no_run
//! use topqueue::page1::*;
//!
//! let (n, m) = (10, 3);
//! let n_rands = make_rands(n);
//! let top_m = get_top(&n_rands, m);
//!
//! assert_eq!(
//!     n_rands.iter().max(),
//!     top_m.iter().max()
//! );
//! ```
//!
//! Same as above, but MOAR.
//!
//! ```no_run
//! use topqueue::page1::*;
//!
//! let (n, m) = (1_000_000, 100);
//! let n_rands = make_rands(n);
//! let top_m = get_top(&n_rands, m);
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
//! use topqueue::page1::*;
//!
//! let (n, m) = (100_000_000, 100);
//! let n_rands = make_rands(n);
//! let top_m = get_top(&n_rands, m);
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

use rand::{Rng, thread_rng};

/// Creates a new Vec<i32> of random numbers of the given size.
pub fn make_rands(len: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.gen::<i32>()).collect()
}

/// Takes a vector `vec` and returns the `top` largest values.
pub fn get_top(vec: &Vec<i32>, top: usize) -> Vec<i32> {
    let mut dupe = vec.to_owned();
    dupe.sort_unstable_by(|a, b| b.cmp(a));
    dupe.into_iter().take(top).collect()
}
