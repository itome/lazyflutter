use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::Mutex;

use crate::{
    action::Action,
    daemon::{self, flutter::FlutterDaemon, io::device::Device},
    session::{
        session::Session,
        session_manager::{self, SessionManager},
    },
    store::{state::State, Store},
    tui::Frame,
};
use color_eyre::eyre::Result;

use super::Component;

pub struct AppsComponent {
    session_manager: Arc<Mutex<SessionManager>>,
    daemon: Arc<FlutterDaemon>,
    devices: Arc<Mutex<Vec<Device>>>,
    is_selected: bool,

    list_state: ListState,
}

impl AppsComponent {
    pub fn new(daemon: Arc<FlutterDaemon>, session_manager: Arc<Mutex<SessionManager>>) -> Self {
        Self {
            session_manager,
            daemon,
            devices: Arc::new(Mutex::new(vec![])),
            list_state: ListState::default().with_selected(Some(0)),
            is_selected: false,
        }
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }

    fn next(&mut self) {
        // let i = match self.list_state.selected() {
        //     Some(i) => {
        //         let sessions = &self.session_manager.lock().unwrap().sessions;
        //         if i >= sessions.len() {
        //             0
        //         } else {
        //             i + 1
        //         }
        //     }
        //     None => 0,
        // };
        // self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        // let i = match self.list_state.selected() {
        //     Some(i) => {
        //         if i == 0 {
        //             let sessions = &self.session_manager.lock().unwrap().sessions;
        //             sessions.len()
        //         } else {
        //             i - 1
        //         }
        //     }
        //     None => 0,
        // };
        // self.list_state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }

    fn run_new_app(&mut self) -> Result<()> {
        // if let Ok(mut session_manager) = self.session_manager.lock() {
        //     session_manager.run_new_app()?;
        // }
        Ok(())
    }
}

impl Component for AppsComponent {
    fn handle_key_events(
        &mut self,
        key: KeyEvent,
        store: Arc<Mutex<Store>>,
    ) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('n') => {
                self.run_new_app()?;
            }
            KeyCode::Up => {
                self.previous();
            }
            KeyCode::Down => {
                self.next();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<()> {
        let default_color = if self.is_selected {
            Color::White
        } else {
            Color::DarkGray
        };
        let enabled_color = if self.is_selected {
            Color::Green
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Apps")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        // let sessions = &self.session_manager.lock().unwrap().sessions;
        let sessions: Vec<Session> = vec![];
        let mut items = sessions
            .iter()
            .enumerate()
            .map(|(index, _)| {
                ListItem::new(format!(" App {} ", index + 1))
                    .style(Style::default().fg(enabled_color))
            })
            .collect::<Vec<_>>();
        items.push(ListItem::new(" ▶ Run new app ").style(Style::default().fg(default_color)));

        let list = List::new(items)
            .block(block)
            .fg(Color::White)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }
}
