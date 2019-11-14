// sprawl6.rs

use std::io::{self, Read, Write};

fn main() {
    // generate the bitstring lookup table
    let mut lookup = [[0u8; 8]; 256];
    for i in 0..256 {
        let s: String = format!("{:08b}", i);
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

        // copy bitstring for each byte in inbuffer into outbuffer
        for (i, b) in inbuffer[0..n].iter().enumerate() {
            outbuffer[i * 8..i * 8 + 8].copy_from_slice(&lookup[*b as usize]);
        }

        // dump outbuffer
        let _ = bufout.write_all(&outbuffer[0..(n*8)]);
    }
}
