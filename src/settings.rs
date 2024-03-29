use crossterm::cursor::SetCursorStyle;

use crate::enums::{LineNumber, ShowTab, SplitHorz, SplitVert};

#[derive(Clone, Copy)]
pub struct Settings {
    pub line_number: LineNumber,
    pub line_number_padding: usize,
    pub show_tabs: ShowTab,
    pub tab_numbering: bool,
    pub split_direction: (SplitVert, SplitHorz),
    pub insert_mode_cursor: SetCursorStyle,
    pub normal_mode_cursor: SetCursorStyle,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            line_number: LineNumber::Relative,
            line_number_padding: 1,
            show_tabs: ShowTab::Multiple,
            tab_numbering: true,
            split_direction: (SplitVert::Right, SplitHorz::Bottom),
            insert_mode_cursor: SetCursorStyle::BlinkingBar,
            normal_mode_cursor: SetCursorStyle::SteadyBlock,
        }
    }
}
