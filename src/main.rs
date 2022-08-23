use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

// This function converts a character to a control byte
fn to_ctrl_byte(c: char) -> u8 {
    c as u8 & 0x1f
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let current_char = b as char;
        println!("{}", current_char);
        
        if b == to_ctrl_byte('w') {
            break;
        } else if current_char.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, current_char);
        }
    }
}
