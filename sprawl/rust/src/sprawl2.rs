#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::needless_range_loop)]

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut lookup = [[0u8; 8]; 256];
    for i in 0..=255 {
        let s = format!("{i:08b}");
        lookup[i].copy_from_slice(s.as_bytes());
    }

    let stdin = io::stdin();
    let bufin = stdin.lock();
    let stdout = io::stdout();
    let mut bufout = stdout.lock();

    for byte in bufin.bytes().flatten() {
        bufout.write_all(&lookup[byte as usize])?;
    }

    Ok(())
}
