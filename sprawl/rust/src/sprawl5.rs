#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::large_stack_arrays, clippy::needless_range_loop)]

use std::io::{self, Read, Write};

const BUF_SIZE: usize = 8_192;

fn main() -> io::Result<()> {
    // generate the bitstring lookup table
    let mut lookup = [[0u8; 8]; 256];

    for i in 0..256 {
        let s: String = format!("{i:08b}");
        lookup[i].copy_from_slice(s.as_bytes());
    }

    let mut bufout = io::stdout().lock();

    // allocate read and write buffers
    let mut inbuffer = [0u8; BUF_SIZE];
    let mut outbuffer = [0u8; BUF_SIZE * 8];

    // fill inbuffer
    while let Ok(n) = io::stdin().lock().read(&mut inbuffer) {
        // quit if we're done reading
        if n == 0 {
            break;
        }

        // copy bitstring for each byte in inbuffer into outbuffer
        for (i, b) in inbuffer[0..n].iter().enumerate() {
            outbuffer[i * 8..i * 8 + 8].copy_from_slice(&lookup[*b as usize]);
        }

        // dump outbuffer
        bufout.write_all(&outbuffer[0..(n * 8)])?;
    }

    Ok(())
}
