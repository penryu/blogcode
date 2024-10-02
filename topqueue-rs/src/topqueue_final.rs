//! A binary heap (aka, priority queue) stores inserted values in sorted order
//! as they're added.
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
use std::iter::from_fn;

/// A collection that retains the largest n items inserted into it.
///
/// Implemented using `std::collections::binary_heap<Reverse<_>>`.
#[derive(Debug)]
pub struct TopQueue<T: Ord> {
    capacity: usize,
    queue: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> TopQueue<T> {
    /// Create a new `TopQueue` that tracks the largest `capacity` number of inserted items.
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

    /// Creates a new `TopQueue` of capacity `capacity` and with the elements of
    /// `iter` pushed into it.
    pub fn from_iter<I: IntoIterator<Item = T>>(capacity: usize, iter: I) -> Self {
        iter.into_iter().fold(TopQueue::new(capacity), |mut q, x| {
            q.push(x);
            q
        })
    }

    /// Returns a Vec of the values contained in the queue in the
    /// order they would be returned by the underlying `binary_heap`.
    ///
    /// Consumes the contents of the queue.
    #[must_use]
    pub fn into_vec(mut self) -> Vec<T> {
        // BinaryHeap doesn't allow draining in sorted order,
        // and the into_iter_sorted() method is unstable.
        // So pop() values one at a time.
        from_fn(|| self.queue.pop().map(|r| r.0)).collect()
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

    /// Attempts to insert the value `item` into the queue.
    ///
    /// If the value is less than the smallest already in the queue, it is ignored.
    pub fn push(&mut self, item: T) {
        let rev_item = Reverse(item);

        // If we're under capacity, just push
        if self.queue.len() < self.capacity {
            self.queue.push(rev_item);
        // If new value is greater than the smallest in the queue, push
        // (The underlying BinaryHeap<Reverse<_>> means the comparison
        // operators are reversed.)
        } else if Some(&rev_item) <= self.queue.peek() {
            self.queue.pop();
            self.queue.push(rev_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::topqueue_final::TopQueue;
    use crate::util::rands;

    const TEXT: &str = "In a castle of Westphalia,
        belonging to the Baron of Thunder-ten-Tronckh, lived a youth, whom
        nature had endowed with the most gentle manners.";

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

    /// This test relies too much on the distribution of the rng,
    /// with a fairly forgiving margin.
    #[test]
    fn topq_can_handle_lots_of_values() {
        let q = TopQueue::from_iter(100, rands().take(1_000_000));
        assert_eq!(100, q.len());
        assert_eq!(100, q.capacity());

        let tops = q.into_vec();
        println!("{tops:?}");
        assert!({
            let min_top = tops.iter().min().unwrap();
            let top_000001 = i32::MAX / 10_00;
            (i32::MAX - min_top) < top_000001
        });
    }

    /// Ensures the queue works with other orderable items, like chars.
    #[test]
    fn topq_handles_generic_ordered_items() {
        let count = 5;
        let q = TopQueue::from_iter(count, TEXT.chars());

        let mut all_chars: Vec<_> = TEXT.chars().collect();
        all_chars.sort_unstable();
        assert!(all_chars.ends_with(&q.into_vec()));
    }
}
