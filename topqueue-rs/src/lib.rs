#![warn(clippy::pedantic)]
#![warn(missing_docs)]

//! This is a Rust port of the code from my series of posts on implementing a
//! custom Priority Queue, originally in Scala[^original].
//!
//! Rust counterparts to some of the naive code samples from [page 1] can be found
//! in [util].
//!
//! A Rust port of the initial implementation of `TopQueue` is in
//! [`topqueue_basic`].
//!
//! The final Rust version, featuring most applicable elements from the final
//! Scala version, is in [`topqueue_final`].
//!
//! [^original]: The original series, "A Scala Interview", can be found [here].
//!
//! [here]: https://lex.penryu.dev/posts/scala-interview-1
//! [page 1]: https://lex.penryu.dev/posts/scala-interview-1
//! [topqueue_basic]: topqueue_basic/index.html
//! [topqueue_final]: topqueue_final/index.html
//! [util]: util/index.html

pub mod util;
pub mod topqueue_basic;
pub mod topqueue_final;
