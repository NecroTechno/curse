use crate::logger::curse_log;

use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::ColorStyle;
use cursive::traits::*;
use cursive::vec::Vec2;
use cursive::Printer;

use rand::seq::SliceRandom;

const AVAILABLE_CHARACTERS: &[u8] = "abcdefghijklmnopqrstuvwxyz1234567890!$%&*_+=-".as_bytes();
const CHARACTER_CELL_LENGTH: usize = 3;
const OCCUPIED_WORKSPACE_HEIGHT: usize = 4;

#[derive(Debug, Clone)]
struct Cell {
    content: String,
}

pub struct WordFinderView {
    words: Vec<String>,
    cells: Vec<Cell>,
    selected_cell_index: usize,
    size: Vec2,
    cells_sorted: bool,
}

fn cell_content_generator() -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        AVAILABLE_CHARACTERS
            .choose_multiple(&mut rng, CHARACTER_CELL_LENGTH - 1)
            .cloned()
            .collect(),
    )
    .unwrap()
}

impl WordFinderView {
    pub fn new(words: Vec<String>) -> Self {
        let mut cells = Vec::new();
        for word in words.iter() {
            let mut iter = word.chars();
            let mut pos = 0;

            while pos < word.len() {
                let mut len = 0;
                for ch in iter.by_ref().take(CHARACTER_CELL_LENGTH - 1) {
                    len += ch.len_utf8();
                }
                let mut chunk = word[pos..pos + len].to_string();
                if chunk.len() == 1 {
                    chunk = format!("{}_", chunk);
                };
                cells.push(Cell {
                    content: chunk.to_string(),
                });
                pos += len;
            }
        }

        if cells.len() < 10000 {
            for _i in 1..10000 - cells.len() {
                cells.push(Cell {
                    content: cell_content_generator(),
                })
            }
        }

        WordFinderView {
            words: words,
            cells: cells,
            selected_cell_index: 0,
            size: Vec2::new(0, 0),
            cells_sorted: false,
        }
    }

    fn update_focus(&mut self, move_index: i32) {
        if move_index.is_negative() {
            if self.selected_cell_index as i32 - (move_index * -1) >= 0 {
                self.selected_cell_index -= (move_index * -1) as usize
            } else {
                self.selected_cell_index = 0;
            }
        } else {
            if self.selected_cell_index as i32 + move_index > self.cells.len() as i32 - 1 {
                self.selected_cell_index = self.cells.len() - 1;
            } else {
                self.selected_cell_index += move_index as usize
            }
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

        printer.print(
            (0, 0),
            &format!("Find the words: {}", self.words.join(", ")),
        );

        printer.print(
            (0, 2),
            "Current entry: ",
        );

        let max_size = self.size.x - CHARACTER_CELL_LENGTH;
        let mut row_size = 0;
        let mut row_count = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            if self.selected_cell_index == i {
                printer.with_color(style, |printer| {
                    printer.print((row_size, (OCCUPIED_WORKSPACE_HEIGHT + row_count)), &cell.content);
                })
            } else {
                printer.print((row_size, (OCCUPIED_WORKSPACE_HEIGHT + row_count)), &cell.content);
            }
            row_size += CHARACTER_CELL_LENGTH;
            if row_size >= max_size {
                row_size = 0;
                row_count += 1;
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let cells_to_move: f32 = ((self.size.x - 1) / CHARACTER_CELL_LENGTH) as f32;
        match event {
            Event::Key(Key::Right) if self.selected_cell_index + 1 < self.cells.len() => {
                self.update_focus(1)
            }
            Event::Key(Key::Left) if self.selected_cell_index > 0 => self.update_focus(-1),
            Event::Key(Key::Down)
                if (self.selected_cell_index + (cells_to_move.floor() as usize))
                    < self.cells.len() =>
            {
                self.update_focus(cells_to_move as i32)
            }
            Event::Key(Key::Up)
                if (self.selected_cell_index) as i32 - (cells_to_move.floor() as usize) as i32
                    >= 0 =>
            {
                self.update_focus(-(cells_to_move as i32))
            }
            Event::Key(Key::Home) => self.selected_cell_index = 0,
            Event::Key(Key::End) => self.selected_cell_index = self.cells.len() - 1,
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
    }

    fn layout(&mut self, size: Vec2) {
        self.size = size;

        if !self.cells_sorted {
            let max_cells = ((size.x - CHARACTER_CELL_LENGTH) / CHARACTER_CELL_LENGTH) as f32
                * (size.y - OCCUPIED_WORKSPACE_HEIGHT) as f32;
            curse_log(&format!("{}", max_cells));
            self.cells
                .drain(max_cells.floor() as usize..self.cells.len());
            let mut rng = rand::thread_rng();
            self.cells.shuffle(&mut rng);

            self.cells_sorted = true;
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}
