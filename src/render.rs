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

/// Call once after the render loop ends, so the user's terminal
/// isn't left with a permanently hidden cursor.
pub fn cleanup(term: &mut impl Write) {
    execute!(term, Show).unwrap();
}

pub fn draw_frame(term: &mut impl Write, ascii: &str) {
    // No Clear here on purpose: clearing then redrawing creates a visible
    // blank flash every frame. Since every frame is the same fixed
    // width/height, just move to the top-left and overwrite in place.
    execute!(term, MoveTo(0, 0)).unwrap();
    write!(term, "{}", ascii).unwrap();
    term.flush().unwrap();
}
