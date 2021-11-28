//! Tests in this file correspond to example code in the second [blog post].
//!
//! [blog post]: https://lex.penryu.dev/posts/scala-interview-2

//! ```
//! use std::collections::binary_heap::BinaryHeap;
//! use topqueue::page1::make_rands;
//!
//! // Generate a vector of 1000 random ints
//! let mut nums = make_rands(1000);
//! // Find the largest value in a linear search
//! let max_num = nums.iter().max().copied();
//!
//! let mut q = BinaryHeap::from(nums);
//! let first = q.pop();
//! let second = q.pop();
//!
//! assert_eq!(first, max_num);
//! assert!(first > second);
//! ```

use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;

/// A collection that retains the largest n items inserted into it.
///
/// Implemented using `std::collections::binary_heap<Reverse<_>>`.
///
#[derive(Debug)]
pub struct TopQueue {
    capacity: usize,
    queue: BinaryHeap<Reverse<i32>>,
}

impl TopQueue {
    /// Create a new TopQueue that tracks the largest
    /// `capacity` number of inserted items.
    ///
    pub fn new(capacity: usize) -> Self {
        TopQueue {
            capacity,
            queue: BinaryHeap::with_capacity(capacity),
        }
    }

    /// Returns the maximum number of values the queue will retain.
    ///
    /// Unlike a binary heap, this value will not change
    ///
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a Vec of the values contained in the queue in the
    /// order they would be returned by the underlying binary_heap.
    ///
    /// Consumes the contents of the queue.
    ///
    pub fn into_vec(self) -> Vec<i32> {
        // BinaryHeap doesn't allow draining in sorted order like Scala's
        // PriorityQueue, and the into_iter_sorted() method is unstable.
        self.queue
            .into_sorted_vec()
            .into_iter()
            .map(|r| r.0)
            .collect()
    }

    /// Returns the number of elements currently in the TopQueue.
    ///
    /// Will always be <= self.capacity.
    ///
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Attempts to insert the value `n` into the queue.
    ///
    /// If the value is less than the smallest already in the queue,
    /// it is ignored.
    ///
    pub fn push(&mut self, n: i32) {
        // If we're under capacity, just push
        if self.queue.len() < self.capacity() {
            self.queue.push(Reverse(n))
        // If new value is greater than the smallest in the queue, push
        // (The underlying BinaryHeap<Reverse<_>> means the comparison
        // operators are reversed.)
        } else if Some(&Reverse(n)) < self.queue.peek() {
            self.queue.pop();
            self.queue.push(Reverse(n))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topq_can_be_empty() {
        let empty_q = TopQueue::new(5);
        assert_eq!(0, empty_q.len());
        assert_eq!(5, empty_q.capacity());
        assert_eq!(0, empty_q.into_vec().len());
    }

    #[test]
    fn topq_can_be_under_capacity() {
        let mut q = TopQueue::new(5);
        for x in vec![1,5,3] {
            q.push(x);
        }
        assert_eq!(3, q.len());
        assert_eq!(5, q.capacity());
        assert_eq!(vec![5,3,1], q.into_vec());
    }

    #[test]
    fn topq_can_be_at_capacity() {
        let mut q = TopQueue::new(5);
        for x in vec![1,5,3,4,2] {
            q.push(x);
        }
        assert_eq!(5, q.len());
        assert_eq!(5, q.capacity());
        assert_eq!(vec![5,4,3,2,1], q.into_vec());
    }

    #[test]
    fn topq_can_accept_values_at_capacity() {
        let mut q = TopQueue::new(5);
        for x in vec![1,5,3,4,2,7,6] {
            q.push(x);
        }
        assert_eq!(5, q.len());
        assert_eq!(5, q.capacity());
        assert_eq!(vec![7,6,5,4,3], q.into_vec());
    }
}
