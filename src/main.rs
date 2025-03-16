extern crate core;

mod model;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App{};
    let result = app.run(terminal);
    ratatui::restore();
    result
}

struct App {
    playlists: Vec<model::Playlist>
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|x| self.render(x))?;
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}