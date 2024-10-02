//! Utility functions and example code.
//!
//! ```
//! # use topqueue::util::*;
//! let n_rands: Vec<i32> = rands().take(10).collect();
//! let top_m = get_top(&n_rands, 3);
//!
//! assert_eq!(n_rands.iter().max(), top_m.iter().max());
//! ```
//!
//! Same as above, but MOAR.
//!
//! ```
//! # use topqueue::util::*;
//! let n_rands: Vec<i32> = rands().take(1_000_000).collect();
//! let top_m = get_top(&n_rands, 100);
//!
//! assert_eq!(n_rands.iter().max(), top_m.iter().max());
//! ```
//!
//! Same method on a rediciously inefficient scale!
//!
//! ```no_run
//! # use topqueue::util::*;
//! let n_rands: Vec<i32> = rands().take(100_000_000).collect();
//! let top_m = get_top(&n_rands, 100);
//!
//! assert_eq!(n_rands.iter().max(), top_m.iter().max());
//! ```

//! ```rust
//! use topqueue::util::{rands, with_type};
//!
//! let mut random_nums: Vec<u8> = rands().take(10).collect();
//! random_nums.sort();
//! let top3 = random_nums.last_chunk::<3>().unwrap();
//!
//! with_type(&random_nums);
//! with_type(&top3);
//! ```

use std::any::type_name;
use std::iter::from_fn;

use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

/// Creates an infinite iterator of random numbers.
pub fn rands<T>() -> impl Iterator<Item = T>
where
    Standard: Distribution<T>,
{
    from_fn(|| Some(thread_rng().gen()))
}

/// `get_top()` implements a brute force method of selecting the
/// largest `top` integers from the given vector.
#[must_use]
pub fn get_top(nums: &[i32], top: usize) -> Vec<i32> {
    let mut dupe = nums.to_vec();
    dupe.sort_unstable_by(|a, b| b.cmp(a));
    dupe.into_iter().take(top).collect()
}

/// Displays the type of the variable passed.
pub fn with_type<T: std::fmt::Debug>(x: &T) -> String {
    format!("{x:?}: {}", type_name::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rands() {
        // The type system really does this for us.
        // If it returns a value, it'll be of the correct type.
        // We can't really _prove_ it's random.
        let mut chars = rands::<char>();
        assert!(chars.next().is_some());
        assert!(chars.next().is_some());
    }

    #[test]
    fn test_get_top() {
        assert_eq!(&[9, 5, 4], &get_top(&[1, 2, 3, 5, 9, 4], 3)[..]);
    }

    #[test]
    fn test_with_type() {
        assert_eq!("5: i32", with_type(&5_i32));
        assert_eq!(
            "\"foo\": alloc::string::String",
            with_type(&String::from("foo"))
        );
    }
}
