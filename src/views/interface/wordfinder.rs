use cursive::traits::*;
use cursive::{Cursive, Printer};
use cursive::direction::Direction;
use cursive::theme::ColorStyle;

struct Cell {
    content: String
}

pub struct WordFinderView {
    words: Vec<String>,
    cells: Vec<Cell>,
    selected_cell_index: usize
}

impl WordFinderView {
    pub fn new(words: Vec<String>) -> Self {
        WordFinderView {
            words: words,
            cells: Vec::new(),
            selected_cell_index: 0,
        }
    }
}

impl View for WordFinderView {
    fn draw(&self, printer: &Printer) {
        let style = if printer.focused {
            ColorStyle::highlight()
        } else {
            ColorStyle::primary()
        };

        printer.print((0, 0), format!("Find the words: {}",
            self.words.join(", ")
        ).as_str());

        printer.with_color(style, |printer| {
            printer.print((0, 1), "test");
        })
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}
