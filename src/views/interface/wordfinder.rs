//use crate::logger::curse_log;

use cursive::direction::Direction;
use cursive::theme::ColorStyle;
use cursive::traits::*;
use cursive::{Cursive, Printer};

use rand::Rng;
use rand::seq::SliceRandom;
use std::cmp;

const AVAILABLE_CHARACTERS: &[u8] = "abcdefghijklmnopqrstuvwxyz1234567890!$%&*_+=-".as_bytes();

#[derive(Debug)]
struct Cell {
    content: String,
}

pub struct WordFinderView {
    words: Vec<String>,
    cells: Vec<Cell>,
    selected_cell_index: usize,
}

fn cell_content_generator() -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(AVAILABLE_CHARACTERS.choose_multiple(&mut rng, 2).cloned().collect()).unwrap()
}

impl WordFinderView {
    pub fn new(words: Vec<String>) -> Self {
        let mut cells = Vec::new();
        for word in words.iter() {
            let mut iter = word.chars();
            let mut pos = 0;

            while pos < word.len() {
                let mut len = 0;
                for ch in iter.by_ref().take(2) {
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

        if cells.len() < 100 {
            for _i in 1..100 - cells.len() {
                cells.push(Cell {
                    content: cell_content_generator(),
                })
            }
        }

        WordFinderView {
            words: words,
            cells: cells,
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

        printer.print(
            (0, 0),
            format!("Find the words: {}", self.words.join(", ")).as_str(),
        );

        let max_size = 50;
        let mut row_size = 0;
        let mut row_count = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            if self.selected_cell_index == i {
                printer.with_color(style, |printer| {
                    printer.print((row_size, (2 + row_count)), &cell.content);
                })
            } else {
                printer.print((row_size, (2 + row_count)), &cell.content);
            }
            row_size += 3;
            if row_size >= max_size {
                row_size = 0;
                row_count += 1;
            }
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}
