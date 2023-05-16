use core::fmt::Debug;
use crossterm::event::KeyCode;
use std::fmt::Formatter;

pub struct Row {
    pub chars: Vec<KeyCode>,
}

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.chars)
    }
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub chars: Vec<Vec<char>>,
    pub current_char: KeyCode,
    pub current_row_index: usize,
    pub current_col_index: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            width: 0,
            height: 0,
            chars: vec![vec![]],
            current_char: KeyCode::Null,
            current_row_index: 0,
            current_col_index: 0,
        }
    }
}
