use crate::logger::curse_log;

use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key, Callback};
use cursive::theme::ColorStyle;
use cursive::traits::*;
use cursive::vec::Vec2;
use cursive::Printer;
use cursive::Cursive;

use rand::seq::SliceRandom;

const AVAILABLE_CHARACTERS: &[u8] = "abcdefghijklmnopqrstuvwxyz1234567890!$%&*_+=-".as_bytes();
const CHARACTER_CELL_LENGTH: usize = 3;
const OCCUPIED_WORKSPACE_HEIGHT: usize = 2;
const MAX_CELL_VEC_LENGTH: usize = 200;
const WORD_LIST: &'static [&'static str] = &[
    "defrag",
    "reboot",
    "cypher",
    "dotcom",
    "netizen_",
    "phreak",
    "publify_",
    "palmtop_",
    "digerati",
    "dox_",
    "daemon",
    "epsilon_",
    "hack",
    "adware",
    "buffer",
    "exploit_",
    "payload_",
    "rootkit_",
    "trojan",
    "worm",
    "botnet",
    "ransom",
    "repo",
    "vulnerable",
];

#[derive(Debug, Clone)]
struct Cell {
    content: String,
}

pub struct WordFinderView {
    words: Vec<String>,
    current_word: String,
    cells: Vec<Cell>,
    selected_cell_index: usize,
    size: Vec2,
    cells_sorted: bool,
    complete_callback: Callback,
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

fn words_generator(difficulty: u8) -> Vec<String> {
    let rng = &mut rand::thread_rng();
    let mut words = Vec::new();
    for _i in 0..difficulty {
        words.push(WORD_LIST.choose(rng).unwrap().to_string());
    }
    words
}

impl WordFinderView {
    pub fn new<F>(difficulty: u8, cb: F) -> Self
    where
       F: 'static + Fn(&mut Cursive)
    {
        let mut cells = Vec::new();

        let words = words_generator(difficulty);

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

        if cells.len() < MAX_CELL_VEC_LENGTH {
            for _i in 1..MAX_CELL_VEC_LENGTH - cells.len() {
                cells.push(Cell {
                    content: cell_content_generator(),
                })
            }
        }

        WordFinderView {
            words: words,
            current_word: String::new(),
            cells: cells,
            selected_cell_index: 0,
            size: Vec2::new(0, 0),
            cells_sorted: false,
            complete_callback: Callback::from_fn(cb),
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

    fn slice_to_current_word(&mut self) {
        self.current_word
            .push_str(&self.cells[self.selected_cell_index].content);
    }

    fn submit_word(&mut self) -> Callback {
        let submission = self.words.iter().position(|x| x == &self.current_word);
        match submission {
            Some(i) => {
                self.words.remove(i);
                self.current_word = String::new();
            }
            None => (),
        };

        if self.words.is_empty() {
            (self.complete_callback.clone())
        } else {
            // must return a callback - probably could be handled better
            Callback::from_fn((|_s| ()))
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

        // TODO: highlight current entry and available words if current word is found?

        printer.print(
            (0, 0),
            &format!("Find the words: {}", self.words.join(", ")),
        );

        printer.print_box((0, self.size.y - 3), (self.size.x, 3), false);

        printer.print((1, self.size.y - 3), "┤ Current Entry ├");

        printer.print((1, self.size.y - 2), &format!("{}", &self.current_word));

        let max_size = self.size.x - CHARACTER_CELL_LENGTH;
        let mut row_size = 0;
        let mut row_count = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            if self.selected_cell_index == i {
                printer.with_color(style, |printer| {
                    printer.print(
                        (row_size, (OCCUPIED_WORKSPACE_HEIGHT + row_count)),
                        &cell.content,
                    );
                })
            } else {
                printer.print(
                    (row_size, (OCCUPIED_WORKSPACE_HEIGHT + row_count)),
                    &cell.content,
                );
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
            Event::Key(Key::Enter) => self.slice_to_current_word(),
            Event::Key(Key::Tab) => return EventResult::Consumed(Some(self.submit_word())),
            Event::Key(Key::Del) => self.current_word = String::new(),
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
    }

    fn layout(&mut self, size: Vec2) {
        self.size = size;

        if !self.cells_sorted {
            // for trimming cell if too many generated
            // let max_cells = ((size.x - CHARACTER_CELL_LENGTH) / CHARACTER_CELL_LENGTH) as f32
            //     * (size.y - OCCUPIED_WORKSPACE_HEIGHT) as f32;
            // self.cells
            //     .drain(max_cells.floor() as usize..self.cells.len());
            let mut rng = rand::thread_rng();
            self.cells.shuffle(&mut rng);

            self.cells_sorted = true;
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}
