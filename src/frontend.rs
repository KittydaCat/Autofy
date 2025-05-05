use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, StatefulWidget, Style, Widget};
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState};

use crate::backend;

#[derive(Debug, Clone)]
enum Screen{

    EditingPlaylists {
        playlists_list_state: ListState,
        // playlist number?

    },

    EditingSources {
        playlists_list_state: ListState,
        source_list_state: ListState,
    },

    EditingFilters {
        playlists_list_state: ListState,
        source_list_state: ListState,
        filter_list_state: ListState,
    },
}

impl Screen {

    fn demote(self) -> Self {

        match self {

            Screen::EditingPlaylists {
                playlists_list_state: _,
            } => {
                panic!()
            },

            Screen::EditingSources {
                playlists_list_state,
                source_list_state: _,
            } => {
                Screen::EditingPlaylists {
                    playlists_list_state,
                }
            },

            Screen::EditingFilters {
                playlists_list_state,
                source_list_state,
                filter_list_state: _,
            } => {
                Screen::EditingSources {
                    playlists_list_state,
                    source_list_state,
                }
            },
        }

    }

    // this maybe should be in App
    // playlists should be
    fn promote(self) -> Self {

        match self {

            Screen::EditingPlaylists {
                playlists_list_state,
            } => {
                Screen::EditingSources {
                    playlists_list_state,
                    source_list_state: Default::default(),
                }
            },

            Screen::EditingSources {
                playlists_list_state,
                source_list_state,
            } => {
                Screen::EditingFilters {
                    playlists_list_state,
                    source_list_state,
                    filter_list_state: Default::default(),
                }
            },

            Screen::EditingFilters { .. } => {
                panic!()
            },
        }

    }

}

impl Default for Screen {
    fn default() -> Self {
        Screen::EditingPlaylists {
            playlists_list_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct App {
    playlists: Vec<backend::Playlist>,
    screen: Screen,
    exiting: bool,
    // playlist_list: Option<List<'a>>,
    // source_list: Option<List<'a>>,
    // filter_list: Option<List<'a>>,
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


    fn get_items(&self) -> [Option<Vec<ListItem>>;3] {

        let mut lists = [const { None }; 3];



        match &self.screen {
            Screen::EditingPlaylists {
                playlists_list_state: _,
            } => {
                let mut playlist_items: Vec<ListItem> = self.playlists
                    .iter()
                    .map(|x| ListItem::from(x.name.as_str()))
                    .collect();

                let add_new = ListItem::new("+add new playlist")
                    .style(Style::from(Color::Cyan));

                playlist_items.push(add_new);

                lists[0] = Some(playlist_items);
            },

            Screen::EditingSources {
                playlists_list_state,
                source_list_state: _,
            } => {

                let playlist_items: Vec<ListItem> = self.playlists
                    .iter()
                    .map(|x| ListItem::from(x.name.as_str()))
                    .collect();

                lists[0] = Some(playlist_items);

                let mut source_items: Vec<ListItem> = self.playlists
                    .get(
                        playlists_list_state
                            .selected()
                            .expect("If we are editing a playlist it should be selected")
                    )
                    .expect("If a playlist is selected it should exist")
                    .sources
                    .iter()
                    .map(|x| ListItem::from(x.name.as_str()))
                    .collect();

                let add_new = ListItem::new("+add new source")
                    .style(Style::from(Color::Cyan));

                source_items.push(add_new);

                lists[1] = Some(source_items);

            },

            Screen::EditingFilters {
                playlists_list_state,
                source_list_state,
                filter_list_state: _,
            } => {

                let playlist_items: Vec<ListItem> = self.playlists
                    .iter()
                    .map(|x| ListItem::from(x.name.as_str()))
                    .collect();

                lists[0] = Some(playlist_items);

                let playlist = &self.playlists
                    .get(
                        playlists_list_state
                            .selected()
                            .expect("If we are editing a playlist it should be selected")
                    )
                    .expect("If a playlist is selected it should exist")
                    .sources;

                let source_items = playlist.iter()
                    .map(|x| ListItem::from(x.name.as_str()))
                    .collect::<Vec<ListItem>>();

                lists[1] = Some(source_items);

                let mut filter_items: Vec<ListItem> = playlist
                    .get(
                        source_list_state
                            .selected()
                            .expect("If we are editing a filter it should be selected")
                    )
                    .expect("If a filter is selected it should exist")
                    .filters
                    .iter()
                    .map(|x| ListItem::from(x.name()))
                    .collect();

                let add_new = ListItem::new("+add new filter")
                    .style(Style::from(Color::Cyan));

                filter_items.push(add_new);

                lists[2] = Some(filter_items);
            },
        }

        lists

    }

}

impl Widget for &mut App {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::new()
            .borders(Borders::ALL)
            .title("Autofy");

        let [playlist_list_items, source_list_items, filter_list_items,] = self.get_items();

        match &mut self.screen {

            Screen::EditingPlaylists {
                playlists_list_state
            } => {

                let playlist_block = Block::new()
                    .borders(Borders::ALL)
                    .title("Playlists");

                let playlist_list = List::new(
                    playlist_list_items.expect("This should always be supplied with this screen setting")
                )
                    .block(playlist_block)
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always);

                StatefulWidget::render(playlist_list, area, buf, playlists_list_state)
            },

            Screen::EditingSources {
                playlists_list_state,
                source_list_state
            } => {
                let playlist_block = Block::new()
                    .borders(Borders::ALL)
                    .title("Playlists");

                let playlist_list = List::new(
                    playlist_list_items.expect("This should always be supplied with this screen setting")
                )
                    .block(playlist_block)
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always);
            },

            Screen::EditingFilters {
                playlists_list_state,
                source_list_state,
                filter_list_state
            } => {},
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