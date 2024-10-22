#![warn(clippy::pedantic)]
#![deny(clippy::all)]

mod fb;

use fb::{FBSet, FBStats, FB};
use std::mem::size_of;

fn main() {
    let test = (1..=15).map(FB::from).collect::<Vec<FB>>();
    dbg!(test);

    let mut fbset = FBSet::new(15);
    dbg!(&fbset);

    fbset.update(30);
    dbg!(&fbset);

    fbset.update(20);
    dbg!(&fbset);

    dbg!(size_of::<[usize; 4]>(), size_of::<FBStats>(),);
}
