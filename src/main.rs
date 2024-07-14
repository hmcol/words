use log::debug;
use log::info;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text, ToLine},
    widgets::{
        block::{Position, Title},
        Block, Borders, List, ListState, Paragraph, StatefulWidget, Widget,
    },
    Frame,
};
use std::fs; // import the debug macro from the log crate
use std::io;

// -----------------------------------------------------------------------------

mod filter;
mod tui;
mod words;

// =============================================================================

// predicates things i need:
// - P(word, letter) -> word contains the given letter
// - P(word, letters) -> word contains each of the given letters at least once
//                       letters with and without repeats perhaps
// - P(word, letters) -> word contains only the given letters

// for a given word, need these predicates:
// - has_letter(letter)
// - has_letters(letters)
// - made_of(letters) = word contains all the given letters and only those letters (with and without repeats) in other words, the word is a permutation/anagram of the given letters. this is good for scrabble and anagrams and anything where you have a specific bank of letters to use

// =============================================================================

fn main() -> io::Result<()> {
    colog::basic_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
    info!("logging initialized");

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

// -----------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    finder: words::WordFinder,
    word_list_state: ListState,
    filter_list_state: ListState,
    selection: bool,
    input_mode: InputMode,
    insert_state: InsertState,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

// key input handling ==========================================================

#[derive(Debug, Default, PartialEq)]
enum InputMode {
    #[default]
    Normal,
    Insert,
}

#[derive(Debug, Default)]
struct InsertState {
    text: String,
}

impl InsertState {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    fn insert(&mut self, c: char) {
        self.text.push(c);
    }

    fn backspace(&mut self) {
        self.text.pop();
    }
}

impl App {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key_event),
            InputMode::Insert => self.handle_insert_mode(key_event),
        }
    }

    fn handle_normal_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.handle_left_arrow(),
            KeyCode::Right => self.handle_right_arrow(),
            KeyCode::Down => self.handle_down_arrow(),
            KeyCode::Up => self.handle_up_arrow(),
            KeyCode::Enter => self.handle_enter(),
            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => {
                self.insert_state.insert(c);
                self.update_insertion();
            }
            KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                self.insert_state.backspace();
                self.update_insertion();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_right_arrow(&mut self) {
        self.selection = !self.selection;
    }

    fn handle_left_arrow(&mut self) {
        self.selection = !self.selection;
    }

    fn handle_down_arrow(&mut self) {
        if self.input_mode != InputMode::Normal {
            return;
        }

        if self.selection {
            self.filter_list_state.select_next();
        } else {
            self.word_list_state.select_next();
        }
    }

    fn handle_up_arrow(&mut self) {
        if self.input_mode != InputMode::Normal {
            return;
        }

        if self.selection {
            self.filter_list_state.select_previous();
        } else {
            self.word_list_state.select_previous();
        }
    }

    fn handle_enter(&mut self) {
        if self.selection {
            let selected = self.filter_list_state.selected().unwrap();
            if selected == self.finder.filters.len() {
                self.finder.add_filter(filter::WordFilter::Length(5));
                // should launch a popup to select the filter type
            } else {
                let s = self.finder.filters[selected].get_string();
                self.insert_state = InsertState::new(&s);
                self.input_mode = InputMode::Insert;
            }
        }
    }

    fn update_insertion(&mut self) {
        let filter_index = self.filter_list_state.selected().unwrap();
        self.finder.filters[filter_index].update(&self.insert_state.text);
    }
}

// rendering ===================================================================

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // outer border and title
        let title = Title::from(" Word Finder ".bold());

        let instructions = Title::from(Line::from(vec![" <Q> ".blue().bold(), " Quit ".into()]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let inner_area = block.inner(area);

        block.render(area, buf);

        // inner content

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Min(1)])
            .split(inner_area);

        // path to word list

        let path_text = Line::from(vec![
            " Word List: ".into(),
            self.finder.file_path.clone().yellow(),
        ]);

        Paragraph::new(path_text)
            .block(Block::new().borders(Borders::BOTTOM))
            .render(layout[0], buf);

        // lower area

        let [left_pane, right_pane] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(layout[1]);

        self.render_words_list(left_pane, buf);
        self.render_filter_list(right_pane, buf);
    }
}

impl App {
    fn render_words_list(&mut self, area: Rect, buf: &mut Buffer) {
        let words: Vec<Line> = self
            .finder
            .iter_filtered_words()
            .map(|w| w.to_line().magenta())
            .collect();

        let mut list = List::new(words).block(Block::bordered().title("Found Words"));

        if !self.selection {
            list = list.highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        }

        StatefulWidget::render(list, area, buf, &mut self.word_list_state);
    }

    fn render_filter_list(&mut self, area: Rect, buf: &mut Buffer) {
        let mut filters: Vec<String> = self.finder.filters.iter().map(|f| f.to_string()).collect();

        filters.push("+ Add Filter".to_string());

        let mut list = List::new(filters).block(Block::bordered().title("Filters"));

        if self.selection {
            let mut style = Style::new().add_modifier(Modifier::REVERSED);

            if self.input_mode == InputMode::Insert {
                style = style.add_modifier(Modifier::ITALIC);
            }

            list = list.highlight_style(style);
        }

        StatefulWidget::render(list, area, buf, &mut self.filter_list_state);
    }
}
