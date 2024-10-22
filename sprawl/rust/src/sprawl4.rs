#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::large_stack_arrays, clippy::needless_range_loop)]

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // generate the bitstring lookup table
    let mut lookup = [[0u8; 8]; 256];

    for i in 0..256 {
        let s = format!("{i:08b}");
        lookup[i].copy_from_slice(s.as_bytes());
    }

    // get buffered handles on stdin and stdout
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut bufin = stdin.lock();
    let mut bufout = stdout.lock();

    // allocate read and write buffers
    let mut inbuffer = [0u8; 4096];
    let mut outbuffer = [0u8; 32768];

    // fill inbuffer
    while let Ok(n) = bufin.read(&mut inbuffer) {
        // quit if we're done reading
        if n == 0 {
            break;
        }

        // write bitstring for each byte in inbuffer
        for (i, b) in inbuffer[0..n].iter().enumerate() {
            for (j, bit) in lookup[*b as usize].iter().enumerate() {
                outbuffer[i * 8 + j] = *bit;
            }
        }

        // dump outbuffer
        bufout.write_all(&outbuffer[0..(n * 8)])?;
    }

    Ok(())
}
