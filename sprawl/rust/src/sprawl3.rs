#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::needless_range_loop)]

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut lookup = [[0u8; 8]; 256];
    for i in 0..256 {
        let s: String = format!("{i:08b}");
        lookup[i].copy_from_slice(s.as_bytes());
    }

    let stdin = io::stdin();
    let mut bufin = stdin.lock();
    let stdout = io::stdout();
    let mut bufout = stdout.lock();

    let mut buffer = [0u8; 4096];
    while let Ok(n) = bufin.read(&mut buffer) {
        if n == 0 {
            break;
        }
        for b in &buffer[0..n] {
            bufout.write_all(&lookup[*b as usize])?;
        }
    }

    Ok(())
}
