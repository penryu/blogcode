// sprawl2.rs
//
use std::io::{self, Read, Write};

fn main() {
    let mut lookup = [[0u8; 8]; 256];
    for i in 0..256 {
        let s: String = format!("{:08b}", i);
        lookup[i].copy_from_slice(s.as_bytes());
    }

    let stdin = io::stdin();
    let bufin = stdin.lock();
    let stdout = io::stdout();
    let mut bufout = stdout.lock();

    for byte in bufin.bytes() {
        if let Ok(b) = byte {
            let _ = bufout.write(&lookup[b as usize]);
        }
    }
}
