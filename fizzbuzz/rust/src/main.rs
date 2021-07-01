mod fb;

use fb::{FB, FBSet, FBStats};
use std::mem::size_of;

fn main() {
    let test = (1..=15).map(|n| FB::from(n)).collect::<Vec<FB>>();
    println!("{:?}", test);

    let mut fbset = FBSet::new(15);
    println!("{:?}", fbset);

    fbset.update(30);
    println!("{:?}", fbset);

    fbset.update(20);
    println!("{:?}", fbset);

    println!("a => {}; s => {}",
        size_of::<[usize; 4]>(),
        size_of::<FBStats>(),
    );
}
