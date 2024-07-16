use crate::{tui, words::WordFinder};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    text::ToLine,
    widgets::{Block, Borders, Clear, List, ListState, Paragraph},
};
use std::io;

// =============================================================================

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    finder: WordFinder,
    state: State,
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

// state =======================================================================

#[derive(Debug)]
struct State {
    word_list: ListState,
    sort_list: ListState,
    pred_list: ListState,
    new_pred_list: ListState,
    focus_pane: SelectableArea,
    input_mode: InputMode,
    insert_buf: String,
}

impl Default for State {
    fn default() -> Self {
        let mut state = Self {
            word_list: Default::default(),
            sort_list: Default::default(),
            pred_list: Default::default(),
            new_pred_list: Default::default(),
            focus_pane: Default::default(),
            input_mode: Default::default(),
            insert_buf: Default::default(),
        };

        state.word_list.select(Some(0));
        state.sort_list.select(Some(0));
        state.pred_list.select(Some(0));

        state
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

impl App {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.state.input_mode {
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
                self.state.insert_buf.push(c);
                self.update_insertion();
            }
            KeyCode::Enter => {
                self.state.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                self.state.insert_buf.pop();
                self.update_insertion();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_right_arrow(&mut self) {
        match self.state.focus_pane {
            SelectableArea::Words => self.state.focus_pane = SelectableArea::Sorting,
            SelectableArea::Sorting => self.state.focus_pane = SelectableArea::Predicates,
            _ => {}
        };
    }

    fn handle_left_arrow(&mut self) {
        match self.state.focus_pane {
            SelectableArea::Predicates => self.state.focus_pane = SelectableArea::Sorting,
            SelectableArea::Sorting => self.state.focus_pane = SelectableArea::Words,
            _ => {}
        };
    }

    fn handle_down_arrow(&mut self) {
        if self.state.input_mode != InputMode::Normal {
            return;
        }

        match self.state.focus_pane {
            SelectableArea::Predicates => self.state.pred_list.select_next(),
            SelectableArea::Sorting => self.state.sort_list.select_next(),
            SelectableArea::Words => self.state.word_list.select_next(),
            SelectableArea::NewPredicate => self.state.new_pred_list.select_next(),
        }
    }

    fn handle_up_arrow(&mut self) {
        if self.state.input_mode != InputMode::Normal {
            return;
        }

        match self.state.focus_pane {
            SelectableArea::Predicates => self.state.pred_list.select_previous(),
            SelectableArea::Sorting => self.state.sort_list.select_previous(),
            SelectableArea::Words => self.state.word_list.select_previous(),
            SelectableArea::NewPredicate => self.state.new_pred_list.select_previous(),
        }
    }

    fn handle_enter(&mut self) {
        match self.state.focus_pane {
            SelectableArea::Predicates => {
                let selected_index = self
                    .state
                    .pred_list
                    .selected()
                    .expect("Failed to get selected predicate");
                if selected_index == self.finder.predicates.len() {
                    self.state.focus_pane = SelectableArea::NewPredicate;
                    self.state.new_pred_list.select(Some(0))
                } else {
                    self.state.insert_buf = self.finder.get_predicate_string(selected_index);
                    self.state.input_mode = InputMode::Insert;
                }
            }
            SelectableArea::NewPredicate => {
                let selected = self
                    .state
                    .new_pred_list
                    .selected()
                    .expect("Failed to get selected predicate");
                self.finder.add_predicate(selected);
                self.state.focus_pane = SelectableArea::Predicates;
            }
            _ => {}
        }
    }

    fn handle_delete(&mut self) {
        if SelectableArea::Predicates == self.state.focus_pane {
            let selected_index = self
                .state
                .pred_list
                .selected()
                .expect("Failed to get selected predicate");
            self.finder.remove_predicate(selected_index);
        }
    }

    fn update_insertion(&mut self) {
        let selected_index = self
            .state
            .pred_list
            .selected()
            .expect("Failed to get selected predicate");
        self.finder
            .update_predicate(selected_index, &self.state.insert_buf);
        self.state.word_list.select(Some(0));
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

        let footer_text = match self.state.input_mode {
            InputMode::Normal => "q: quit | ←/→: switch panes | ↑/↓: select | ↵: edit predicate",
            InputMode::Insert => " ←: backspace | ↵: save",
        };

        Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .render(footer, buf);

        // popup - possibly render a popup on top of everything

        if self.state.focus_pane == SelectableArea::NewPredicate {
            let popup_area = centered_rect(area, 50, 50);

            Clear.render(popup_area, buf);

            let popup_block = Block::default()
                .title("New Predicate")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);

            let new_predicates: Vec<Line> = self
                .finder
                .iter_predicate_names()
                .map(|s| s.to_line())
                .collect();

            let list = List::new(new_predicates)
                .block(popup_block)
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

            StatefulWidget::render(list, popup_area, buf, &mut self.state.new_pred_list);
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

        if self.state.focus_pane == SelectableArea::Words {
            list = list.highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        }

        StatefulWidget::render(list, area, buf, &mut self.state.word_list);
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

        if self.state.focus_pane == SelectableArea::Sorting {
            list = list.highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        }

        StatefulWidget::render(list, area, buf, &mut self.state.sort_list);
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

        if self.state.focus_pane == SelectableArea::Predicates {
            let mut style = Style::new().add_modifier(Modifier::REVERSED);

            if self.state.input_mode == InputMode::Insert {
                style = style.add_modifier(Modifier::ITALIC);
            }

            list = list.highlight_style(style);
        }

        StatefulWidget::render(list, area, buf, &mut self.state.pred_list);
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
