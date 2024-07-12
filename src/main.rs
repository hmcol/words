use log::debug;
use log::info;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::ToLine;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListState;
use ratatui::widgets::StatefulWidget;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.handle_left_arrow(),
            KeyCode::Right => self.handle_right_arrow(),
            KeyCode::Down => self.handle_down_arrow(),
            KeyCode::Up => self.handle_up_arrow(),
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
        if self.selection {
            self.filter_list_state.select_next();
        } else {
            self.word_list_state.select_next();
        }
    }

    fn handle_up_arrow(&mut self) {
        if self.selection {
            self.filter_list_state.select_previous();
        } else {
            self.word_list_state.select_previous();
        }
    }
}

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
            list = list.highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        }

        StatefulWidget::render(list, area, buf, &mut self.filter_list_state);
    }
}
