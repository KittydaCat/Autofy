use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState};

use crate::backend;

#[derive(Debug, Clone)]
enum Screen{
    Main {state: ListState},
    Editing {selected_source: usize, state: ListState},
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Main { state: Default::default()}
    }
}

#[derive(Debug, Clone, Default)]
struct App {
    playlists: Vec<backend::Playlist>,
    screen: Screen,
    selected_playlist: usize,
    exiting: bool,
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| f.render_widget(&mut *self, f.area()))?;
            match event::read()? {
                _ => {},
            }
        }

        println!("hello?");
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }
}

impl Widget for &mut App {

    fn render(self, area: Rect, buf: &mut Buffer,) {
        match &mut self.screen {
            Screen::Main {state} => {
                let block = Block::new()
                    .borders(Borders::ALL)
                    .title("Autofy");

                let items: Vec<ListItem> = self.playlists
                    .iter()
                    .map(|x| ListItem::from(x.name.as_str()))
                    .collect();

                let list = List::new(items)
                    .block(block)
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always);

                StatefulWidget::render(list, area, buf, state);
            }
            Screen::Editing {selected_source, state} => {}
        }


    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::default();
    let result = app.run(terminal);
    ratatui::restore();
    result
}