use crate::Terminal;

use termion::event::Key;


const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_postion: Position,
}



impl Editor {

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to Initialize terminal"),
            cursor_postion: Position { x: 0, y: 0 }
        }
    }

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
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            print!("Goodbye.\r");
        } else {
            Terminal::cursor_position(&self.cursor_postion);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key();
        match pressed_key {
            Ok(Key::Ctrl('c')) => self.should_quit = true,
            Ok(Key::Up) 
            | Ok(Key::Down) 
            | Ok(Key::Left) 
            | Ok(Key::Right)
            | Ok(Key::PageUp)
            | Ok(Key::PageDown)
            | Ok(Key::End)
            | Ok(Key::Home) => {
                self.move_cursor(pressed_key.unwrap());
            },
            _ => (),
        }
        Ok(())
    }
    fn move_cursor(&mut self, key: Key){
        let Position {mut x, mut y } = self.cursor_postion;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }}
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }}
            _ => (),
        }
        self.cursor_postion = Position { x, y };
    }

    fn draw_welcome_message(&self) {            
            let mut welcome_message = format!("Hecto editor -- version {}", VERSION);            
            let width = self.terminal.size().width as usize;            
            let len = welcome_message.len();            
            let padding = width.saturating_sub(len) / 2;            
            let spaces = " ".repeat(padding.saturating_sub(1));            
            welcome_message = format!("~{}{}", spaces, welcome_message);            
            welcome_message.truncate(width);            
            println!("{}\r", welcome_message);            
        }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1{
            Terminal::clear_current_line();
            if row == height/3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}


// This function takes an error and panics, ending the program
fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}


