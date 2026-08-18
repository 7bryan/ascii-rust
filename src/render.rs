use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use std::io::Write;

pub fn draw_frame(term: &mut impl Write, ascii: &str) {
    execute!(term, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    write!(term, "{}", ascii).unwrap();
    term.flush().unwrap();
}
