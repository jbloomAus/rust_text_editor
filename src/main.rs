use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let input = b.as_ref().unwrap();
        let current_char = *input as char;
        println!("{}", current_char);
        
        if current_char == 'q' {
            break;
        } else if current_char.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, current_char);
        }
    }
}
