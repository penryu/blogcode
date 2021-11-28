//! Tests in this file correspond to example code in the second [blog post].
//!
//! [blog post]: https://lex.penryu.dev/posts/scala-interview-2

//! A binary heap (aka, priority queue) stores inserted values in sorted order
//! as they're added.
//!
//! ```
//! # use std::collections::binary_heap::BinaryHeap;
//! # use topqueue::page1::make_rands;
//! // Generate a vector of 1000 random ints
//! let mut nums: Vec<i32> = make_rands(1000).collect();
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
/// Implemented using `std::collections::binary_heap<Reverse<_>>`.
///
#[derive(Debug)]
pub struct TopQueue {
    size: usize,
    queue: BinaryHeap<Reverse<i32>>,
}

impl TopQueue {
    /// Create a new TopQueue that tracks the largest
    /// `size` number of inserted items.
    ///
    pub fn new(size: usize) -> Self {
        TopQueue {
            size,
            queue: BinaryHeap::with_capacity(size),
        }
    }

    /// Creates a new TopQueue of size `size` and with the elements of
    /// `iter` pushed into it.
    pub fn from_iter<I: IntoIterator<Item=i32>>(size: usize, iter: I) -> Self {
        let mut q = TopQueue::new(size);

        for x in iter {
            q.push(x);
        }

        q
    }
    /// Returns the maximum number of values the queue will retain.
    ///
    /// Unlike a binary heap, this value will not change
    ///
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a Vec of the values contained in the queue in the
    /// order they would be returned by the underlying binary_heap.
    ///
    /// Consumes the contents of the queue.
    ///
    pub fn into_vec(mut self) -> Vec<i32> {
        // BinaryHeap doesn't allow draining in sorted order like Scala's
        // PriorityQueue, and the into_iter_sorted() method is unstable.
        self.queue
            .drain()
            .map(|r| r.0)
            .collect()
    }

    /// Returns the number of elements currently in the TopQueue.
    ///
    /// Will always be <= `self.size`.
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
        if self.queue.len() < self.size() {
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

// impl FromIterator<i32> for TopQueue {
//     fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
//         let mut q = TopQueue::new(10);

//         for x in iter {
//             q.push(x);
//         }

//         q
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page1::make_rands;

    #[test]
    fn topq_basics() {
        let mut q = TopQueue::new(10);
        assert_eq!(0, q.len());
        assert_eq!(10, q.size());

        // Throw in a handful of values
        for n in vec![1, 3, 5, 7, 21, 23] {
            q.push(n);
        }
        assert_eq!(6, q.len());
        assert_eq!(10, q.size());

        // Push more than the queue will hold
        for n in vec![4, 6, 12, 14, 18, 20] {
            q.push(n);
        }
        assert_eq!(10, q.len());
        assert_eq!(10, q.size());

        let mut output = q.into_vec();
        output.sort_by(|a, b| b.cmp(a));
        assert_eq!(output, vec![23, 21, 20, 18, 14, 12, 7, 6, 5, 4]);
    }

    #[test]
    fn topq_can_handle_lots_of_values() {
        let q = TopQueue::from_iter(100, make_rands(1_000_000_000));
        let tops = q.into_vec();
        println!("{:?}", tops);
        assert!({
            let min_top = tops.iter().min().unwrap();
            let top_000001 = i32::MAX / 100_000;
            (i32::MAX - min_top) < top_000001
        });
    }
}