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
    Main,
    Editing {source_list_state: ListState},
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Main
    }
}

#[derive(Debug, Clone, Default)]
struct App {
    playlists: Vec<backend::Playlist>,
    screen: Screen,
    main_list_state: ListState,
    exiting: bool,
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| f.render_widget(&mut *self, f.area()))?;

            // todo change to poll once background processes are running
            match event::read()? {
                Event::Key(KeyEvent{
                               code,
                               modifiers: _,
                               kind: KeyEventKind::Press,
                               state: _ ,
                           }) => {
                    self.handle(code)
                }
                _ => {},
            }

            if self.exiting {break}
        }

        Ok(())
    }

    fn handle(&mut self, code: KeyCode) {
        match code {
            KeyCode::Backspace => {}
            KeyCode::Enter => {}
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Tab => {}
            KeyCode::Delete => {}
            KeyCode::Char('q') => {self.exiting = true}
            KeyCode::Char(_) => {}
            KeyCode::Esc => {}
            x => {dbg!(code);}
        }
    }
}

impl Widget for &mut App {

    fn render(self, area: Rect, buf: &mut Buffer,) {
        match &mut self.screen {
            Screen::Main => {
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

                StatefulWidget::render(list, area, buf, &mut self.main_list_state);
            }
            Screen::Editing {source_list_state} => {}
        }


    }
}

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::default();
    let result = app.run(terminal);
    ratatui::restore();
    result
}