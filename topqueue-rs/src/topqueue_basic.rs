//! A binary heap (aka, priority queue) that stores inserted
//! `i32` values in sorted order as they're added.
//!
//! ```
//! # use std::collections::binary_heap::BinaryHeap;
//! # use topqueue::util::rands;
//! // Generate a vector of 1000 random ints
//! let mut nums: Vec<i32> = rands().take(1000).collect();
//! // Find the largest value in a linear search
//! let max_num = nums.iter().max().copied();
//!
//! let mut q = BinaryHeap::from(nums);
//! let (first, second) = (q.pop(), q.pop());
//!
//! assert_eq!(first, max_num);
//! assert!(first > second);
//! ```

use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;

/// A collection that retains the largest n items inserted into it.
///
/// Implemented using `std::collections::binary_heap<Reverse<i32>>`.
#[derive(Debug)]
pub struct TopQueue {
    capacity: usize,
    queue: BinaryHeap<Reverse<i32>>,
}

impl TopQueue {
    /// Create a new `TopQueue` that tracks the largest
    /// `capacity` number of inserted items.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        TopQueue {
            capacity,
            queue: BinaryHeap::with_capacity(capacity),
        }
    }

    /// Returns the maximum number of values the queue will retain.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a Vec of the values contained in the queue in the
    /// order they would be returned by the underlying `binary_heap`.
    ///
    /// Consumes the contents of the queue.
    #[must_use]
    pub fn into_vec(mut self) -> Vec<i32> {
        // BinaryHeap doesn't allow draining in sorted order,
        // and the into_iter_sorted() method is unstable.
        // So pop() values one at a time.
        self.queue.drain().map(|r| r.0).collect()
    }

    /// Returns true if the underlying queue length is 0.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }

    /// Returns the number of elements currently in the `TopQueue`.
    ///
    /// Will always be <= `self.capacity`.
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Attempts to insert the value `n` into the queue.
    ///
    /// If the value is less than the smallest already in the queue, it is ignored.
    pub fn push(&mut self, n: i32) {
        let rev_n = Reverse(n);

        // If we're under capacity, just push
        if self.queue.len() < self.capacity {
            self.queue.push(rev_n);
        // If new value is greater than the smallest in the queue, push
        // (The underlying BinaryHeap<Reverse<_>> means the comparison
        // operators are reversed.)
        } else if Some(&rev_n) < self.queue.peek() {
            self.queue.pop();
            self.queue.push(rev_n);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::topqueue_basic::TopQueue;
    use crate::util::rands;

    #[test]
    fn topq_basics() {
        let mut q = TopQueue::new(10);
        assert_eq!(0, q.len());
        assert_eq!(10, q.capacity());

        // Throw in a handful of values
        for n in &[1, 3, 5, 7, 21, 23] {
            q.push(*n);
        }
        assert_eq!(6, q.len());
        assert_eq!(10, q.capacity());

        // Push more than the queue will hold
        for n in &[4, 6, 12, 14, 18, 20] {
            q.push(*n);
        }
        assert_eq!(10, q.len());
        assert_eq!(10, q.capacity());

        let mut output = q.into_vec();
        output.sort_by(|a, b| b.cmp(a));
        assert_eq!(output, vec![23, 21, 20, 18, 14, 12, 7, 6, 5, 4]);
    }

    /// This test mostly just tests the distribution of the rng,
    /// with a fairly forgiving margin.
    #[test]
    fn topq_can_handle_lots_of_values() {
        let mut q = TopQueue::new(100);
        for n in rands().take(10_000_000) {
            q.push(n);
        }
        assert_eq!(100, q.len());
        assert_eq!(100, q.capacity());

        let tops = q.into_vec();
        println!("{tops:?}");
        assert!({
            let min_top = tops.iter().min().unwrap();
            let top_000001 = i32::MAX / 10_000;
            (i32::MAX - min_top) < top_000001
        });
    }
}
