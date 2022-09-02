use std::{fs::File, io::Read};

use types::U8Iter;

mod types;

// Currently read only

fn main() {
    let mut file = File::open("message.mrsx").unwrap();
    let mut buf = [0u8];

    while file.read_exact(&mut buf).is_ok() {
        let byte = U8Iter::new(buf[0]);

        for c in byte.into_morse() {
            print!("{c}");
        }
    }

    println!();
}
