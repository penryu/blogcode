#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![warn(missing_docs)]

//! This is a Rust port of the code from my series of posts on implementing a
//! custom Priority Queue, originally in Scala[^original].
//!
//! Rust counterparts to some of the code samples from [page 1][scala_interview_1] can be
//! found in [util].
//!
//! A Rust port of the initial "naive" implementation of `TopQueue` is in [`topqueue_basic`].
//!
//! The final Rust version, featuring most applicable elements from the final
//! Scala version, is in [`topqueue_final`].
//!
//! [^original]: The original series, "A Scala Interview", can be found [here][scala_interview_1].
//!
//! [scala_interview_1]: https://blog.pun.ninja/scala-interview-1
//! [topqueue_basic]: topqueue_basic/index.html
//! [topqueue_final]: topqueue_final/index.html
//! [util]: util/index.html

pub mod topqueue_basic;
pub mod topqueue_final;
pub mod util;
