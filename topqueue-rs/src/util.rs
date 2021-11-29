//! Utility functions and example code.
//!
//! ```
//! # use topqueue::util::*;
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
//! # use topqueue::util::*;
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
//! # use topqueue::util::*;
//! let n_rands: Vec<i32> = make_rands(100_000_000).collect();
//! let top_m = get_top(&n_rands, 100);
//!
//! assert_eq!(
//!     n_rands.iter().max(),
//!     top_m.iter().max()
//! );
//! ```

use std::iter::from_fn;
use rand::{Rng, thread_rng};

/// Creates a new iterator of random numbers of the given size.
pub fn make_rands(len: usize) -> impl Iterator<Item=i32> {
    from_fn(move || Some(thread_rng().gen())).take(len)
}

/// `get_top()` implements a brute force method of selecting the
/// largest `top` integers from the given vector.
#[must_use]
pub fn get_top(nums: &[i32], top: usize) -> Vec<i32> {
    let mut dupe = nums.to_owned().into_iter().collect::<Vec<i32>>();
    dupe.sort_unstable_by(|a, b| b.cmp(a));
    dupe.into_iter().take(top).collect()
}
