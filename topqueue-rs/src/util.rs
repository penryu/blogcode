//! Utility functions and example code.
//!
//! ```
//! use std::any::type_name_of_val as type_of;
//!
//! use topqueue::util::rands;
//!
//! let mut random_nums: Vec<u8> = rands().take(10).collect();
//! random_nums.sort();
//! let top3 = random_nums.last_chunk::<3>().unwrap();
//!
//! println!("{random_nums:?}: {}", type_of(&random_nums));
//! println!("{top3:?}: {}", type_of(&top3));
//! ```
//!
//! Same as above, but MOAR.
//!
//! ```
//! # use std::any::type_name_of_val as type_of;
//! # use topqueue::util::rands;
//! let mut nums: Vec<u16> = rands().take(10_000).collect();
//! nums.sort_unstable();
//! let top5 = nums.last_chunk::<5>().unwrap();
//!
//! println!("{top5:?}: {}", type_of(&top5));
//! ```
//!
//! Same method on a ridiculously inefficient scale!
//!
//! ```
//! # use std::any::type_name_of_val as type_of;
//! # use topqueue::util::rands;
//! let mut nums: Vec<u32> = rands().take(1_000_000).collect();
//! nums.sort_unstable();
//! let top100 = nums.last_chunk::<100>().unwrap();
//!
//! println!("{top100:?}: {}", type_of(&top100));
//! ```

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

#[cfg(test)]
mod tests {
    use std::any::type_name_of_val as type_of;

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
        assert_eq!("i32", type_of(&5_i32));
        assert_eq!("alloc::string::String", type_of(&String::from("foo")));
    }

    #[test]
    fn test_top3() {
        let mut nums: Vec<u8> = rands().take(10).collect();
        nums.sort_unstable();
        let top3 = nums.last_chunk::<3>().unwrap();

        assert_eq!("alloc::vec::Vec<u8>", type_of(&nums));
        assert_eq!("&[u8; 3]", type_of(&top3));
    }

    #[test]
    fn test_top100() {
        let mut nums: Vec<i32> = rands().take(1_000_000).collect();
        nums.sort_unstable();
        let top100 = nums.last_chunk::<100>().unwrap();

        assert_eq!("&[i32; 100]", type_of(&top100));
    }
}
