use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn read_morse() -> String {
    let mut file = File::open("message.mrs").unwrap();
    let mut buf = vec![];
    let mut result_string = String::new();
    let mut is_one = false;
    let mut zeros = 0u8;
    if let Ok(_) = file.read_to_end(&mut buf) {
        for b in buf {
            if b == 1 {
                match zeros {
                    2 => result_string.push(' '),
                    3 => result_string.push_str(" / "),
                    _ => (),
                }
                zeros = 0;
            }

            match b {
                1 if is_one => {
                    result_string.push('-');
                    is_one = false
                }
                1 => is_one = true,
                0 if is_one => {
                    result_string.push('.');
                    is_one = false;
                    zeros += 1
                }
                0 => zeros += 1,
                _ => unreachable!(),
            }
        }
    }
    result_string
}

fn write_morse() {
    let mut file = OpenOptions::new().write(true).open("message.mrs").unwrap();

    file.write_all(&[
        1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1,
        0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1,
        0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1,
    ])
    .unwrap();
}

fn main() {
    // HELLO WORLD
    // .... . .-.. .-.. --- / .-- --- .-. .-.. -..
    // 1010101 00 1 00 10110101 00 10110101 00 11011011 000 1011011 00 11011011 00 101101 00 10110101 00 110101

    write_morse();

    println!("{}", read_morse());
}
