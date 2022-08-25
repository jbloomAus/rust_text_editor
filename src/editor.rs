use crate::Document;
use crate::Terminal;
use crate::Row;
use std::env;
use termion::event::Key;


const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
}



impl Editor {

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            Document::open(&args[1]).unwrap()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to Initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default()
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
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            print!("Goodbye.\r");
        } else {
            Terminal::cursor_position(&Position {
                 x: self.cursor_position.x.saturating_sub(self.offset.x), 
                 y: self.cursor_position.y.saturating_sub(self.offset.y) 
                });
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
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key){
        let Position {mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
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
        self.cursor_position = Position { x, y };
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

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let width = self.terminal.size().width as usize;
        let end = start + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1{
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height/3 {
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


