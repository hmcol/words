use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    text::ToLine,
    widgets::{Block, Borders, Clear, List, ListState, Paragraph},
};
use std::io;

// -----------------------------------------------------------------------------

mod pred;
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
    log::info!("logging initialized");

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

// -----------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    finder: words::WordFinder,
    word_list_state: ListState,
    sort_list_state: ListState,
    predicate_list_state: ListState,
    new_predicate_list_state: ListState,
    selected_area: SelectableArea,
    input_mode: InputMode,
    insert_state: InsertState,
}

impl App {
    fn init(&mut self) {
        self.word_list_state.select(Some(0));
        self.sort_list_state.select(Some(0));
        self.predicate_list_state.select(Some(0));
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.init();
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
enum SelectableArea {
    #[default]
    Predicates,
    Words,
    NewPredicate,
    Sorting,
}

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
            KeyCode::Delete => self.handle_delete(),
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
        match self.selected_area {
            SelectableArea::Words => self.selected_area = SelectableArea::Sorting,
            SelectableArea::Sorting => self.selected_area = SelectableArea::Predicates,
            _ => {}
        };
    }

    fn handle_left_arrow(&mut self) {
        match self.selected_area {
            SelectableArea::Predicates => self.selected_area = SelectableArea::Sorting,
            SelectableArea::Sorting => self.selected_area = SelectableArea::Words,
            _ => {}
        };
    }

    fn handle_down_arrow(&mut self) {
        if self.input_mode != InputMode::Normal {
            return;
        }

        match self.selected_area {
            SelectableArea::Predicates => self.predicate_list_state.select_next(),
            SelectableArea::Sorting => self.sort_list_state.select_next(),
            SelectableArea::Words => self.word_list_state.select_next(),
            SelectableArea::NewPredicate => self.new_predicate_list_state.select_next(),
        }
    }

    fn handle_up_arrow(&mut self) {
        if self.input_mode != InputMode::Normal {
            return;
        }

        match self.selected_area {
            SelectableArea::Predicates => self.predicate_list_state.select_previous(),
            SelectableArea::Sorting => self.sort_list_state.select_previous(),
            SelectableArea::Words => self.word_list_state.select_previous(),
            SelectableArea::NewPredicate => self.new_predicate_list_state.select_previous(),
        }
    }

    fn handle_enter(&mut self) {
        match self.selected_area {
            SelectableArea::Predicates => {
                let selected_index = self.predicate_list_state.selected().expect("Failed to get selected predicate");
                if selected_index == self.finder.predicates.len() {
                    self.selected_area = SelectableArea::NewPredicate;
                    self.new_predicate_list_state.select(Some(0))
                } else {
                    let s = self.finder.get_predicate_string(selected_index);
                    self.insert_state = InsertState::new(&s);
                    self.input_mode = InputMode::Insert;
                }
            }
            SelectableArea::NewPredicate => {
                let selected = self.new_predicate_list_state.selected().expect("Failed to get selected predicate");
                self.finder.add_predicate(selected);
                self.selected_area = SelectableArea::Predicates;
            }
            _ => {}
        }
    }

    fn handle_delete(&mut self) {
        if SelectableArea::Predicates == self.selected_area {
            let selected_index = self
                .predicate_list_state
                .selected()
                .expect("Failed to get selected predicate");
            self.finder.remove_predicate(selected_index);
        }
    }

    fn update_insertion(&mut self) {
        let selected_index = self
            .predicate_list_state
            .selected()
            .expect("Failed to get selected predicate");
        self.finder
            .update_predicate(selected_index, &self.insert_state.text);
        self.word_list_state.select(Some(0));
    }
}

// rendering ===================================================================

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header, subheader, content, footer] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .areas(area);

        // header - app name

        let heading = "Word Finder".to_string().bold();
        Paragraph::new(heading)
            .alignment(Alignment::Center)
            .render(header, buf);

        // subheader - file path to word list

        let path_text = Line::from(vec![
            " Word List: ".into(),
            self.finder.file_path.clone().yellow(),
        ]);

        Paragraph::new(path_text)
            .block(Block::new().borders(Borders::ALL))
            .render(subheader, buf);

        // content - list of words and predicates

        let [left_pane, middle_pane, right_pane] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .areas(content);

        self.render_words_pane(left_pane, buf);
        self.render_sorting_pane(middle_pane, buf);
        self.render_predicate_pane(right_pane, buf);

        // footer - controls

        let footer_text = match self.input_mode {
            InputMode::Normal => "q: quit | ←/→: switch panes | ↑/↓: select | ↵: edit predicate",
            InputMode::Insert => " ←: backspace | ↵: save",
        };

        Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .render(footer, buf);

        // popup - possibly render a popup on top of everything

        if self.selected_area == SelectableArea::NewPredicate {
            let popup_area = centered_rect(area, 50, 50);

            Clear.render(popup_area, buf);

            let popup_block = Block::default()
                .title("New Predicate")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);

            let new_predicates: Vec<Line> = pred::PREDICATES.iter().map(|s| s.to_line()).collect();

            let list = List::new(new_predicates)
                .block(popup_block)
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

            StatefulWidget::render(list, popup_area, buf, &mut self.new_predicate_list_state);
        }
    }
}

impl App {
    fn render_words_pane(&mut self, area: Rect, buf: &mut Buffer) {
        let words: Vec<Line> = self
            .finder
            .iter_filtered()
            .map(|w| w.to_line().magenta())
            .collect();

        let mut list = List::new(words).block(
            Block::bordered()
                .title("Found Words")
                .title_alignment(Alignment::Center),
        );

        if self.selected_area == SelectableArea::Words {
            list = list.highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        }

        StatefulWidget::render(list, area, buf, &mut self.word_list_state);
    }

    fn render_sorting_pane(&mut self, area: Rect, buf: &mut Buffer) {
        let sorting_options = vec![
            "alphabetical".to_line(),
            "reverse alphabetical".to_line(),
            "longest -> shortest".to_line(),
            "shortest -> longest".to_line(),
        ];

        // let words: Vec<Line> = self
        //     .finder
        //     .iter_filtered()
        //     .map(|w| w.to_line().magenta())
        //     .collect();

        let mut list = List::new(sorting_options).block(
            Block::bordered()
                .title("Sorting")
                .title_alignment(Alignment::Center),
        );

        if self.selected_area == SelectableArea::Sorting {
            list = list.highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        }

        StatefulWidget::render(list, area, buf, &mut self.sort_list_state);
    }

    fn render_predicate_pane(&mut self, area: Rect, buf: &mut Buffer) {
        let mut predicates: Vec<String> = self
            .finder
            .predicates
            .iter()
            .map(|f| f.to_string())
            .collect();

        predicates.push("+ New Predicate".to_string());

        let mut list = List::new(predicates).block(
            Block::bordered()
                .title("Predicates")
                .title_alignment(Alignment::Center),
        );

        if self.selected_area == SelectableArea::Predicates {
            let mut style = Style::new().add_modifier(Modifier::REVERSED);

            if self.input_mode == InputMode::Insert {
                style = style.add_modifier(Modifier::ITALIC);
            }

            list = list.highlight_style(style);
        }

        StatefulWidget::render(list, area, buf, &mut self.predicate_list_state);
    }
}

// =============================================================================

/// A helper function to create a centered rectangle within the given area
///
/// taken from the ratatui book
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
