#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod document;
mod row;
mod terminal;
use editor::Editor;
pub use document::Document;

pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
