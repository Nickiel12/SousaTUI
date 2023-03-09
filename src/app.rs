use std::error;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::layout::{Constraint, Direction, Layout};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/latest/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/master/examples
        let root_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20)
                ].as_ref()
            )
            .split(frame.size());

        let top_split = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ].as_ref()
            )
            .split(root_layout[0]);

        let right_split = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref()
            )
            .split(top_split[1]);

        let bottom_block = Block::default()
            .title("bottom block")
            .borders(Borders::ALL);
        frame.render_widget(bottom_block, root_layout[1]);

        frame.render_widget(
            Paragraph::new(
                "This is a tui-rs template.\nPress `Esc`, `Ctrl-C` or `q` to stop running.",
            ).block(
                Block::default()
                    .title("left top block")
                    .borders(Borders::ALL),
            ),
            top_split[0]);

        let top_right_block = Block::default()
            .title("top right block")
            .borders(Borders::ALL);
        frame.render_widget(top_right_block, right_split[0]);

        let right_bottom_block = Block::default()
            .title("right bottom block")
            .borders(Borders::ALL);
        frame.render_widget(right_bottom_block, right_split[1]);

        /*
        frame.render_widget(
            Paragraph::new(
                "This is a tui-rs template.\nPress `Esc`, `Ctrl-C` or `q` to stop running.",
            )
            .block(
                Block::default()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
            frame.size(),
        )
        */
    }
}
