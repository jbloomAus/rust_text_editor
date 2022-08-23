use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

// This function converts a character to a control byte
fn to_ctrl_byte(c: char) -> u8 {
    c as u8 & 0x1f
}

// This function takes an error and panics, ending the program
fn die(e: io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {

        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('c') {
                    break;
                }
            }
            Err(e) => die(e),
        }

       
    }
}
