// sprawl3.rs

use std::io::{self, Read, Write};

fn main() {
    let mut lookup = [[0u8; 8]; 256];
    for i in 0..256 {
        let s: String = format!("{:08b}", i);
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
            let _ = bufout.write(&lookup[*b as usize]);
        }
    }
}
