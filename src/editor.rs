use crate::Terminal;

use termion::event::Key;


pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            } else {
                self.draw_rows();
                print!("{}", termion::cursor::Goto(1, 1))
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0,0);
        if self.should_quit {
            print!("Goodbye.\r");
        } else {
            Terminal::cursor_position(0,0);
        }
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key();
        match pressed_key {
            Ok(Key::Ctrl('c')) => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to Initialize terminal"),
        }
    }
}

// This function takes an error and panics, ending the program
fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}


