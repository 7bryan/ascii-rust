use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType},
};

use std::io::Write;

// clearing stale content already on the screen only once.
pub fn init(term: &mut impl Write) {
    execute!(term, Clear(ClearType::All), Hide).unwrap();
}

/// call only once.
pub fn cleanup(term: &mut impl Write) {
    execute!(term, Show).unwrap();
}

pub fn draw_frame(term: &mut impl Write, ascii: &str) {
    // removing Clear
    execute!(term, MoveTo(0, 0)).unwrap();
    write!(term, "{}", ascii).unwrap();
    term.flush().unwrap();
}
