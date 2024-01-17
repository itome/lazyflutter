use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use crate::{
    action::Action,
    daemon::{self, flutter::FlutterDaemon, io::device::Device},
    session::session_manager::{self, SessionManager},
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
            list_state: ListState::default(),
            is_selected: false,
        }
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}

impl Component for AppsComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if key.code == KeyCode::Char('n') {
            if let Ok(mut session_manager) = self.session_manager.lock() {
                session_manager.run_new_app()?;
            }
        }
        Ok(None)
    }

    fn init(&mut self, area: Rect) -> Result<()> {
        let daemon = self.daemon.clone();
        let devices = self.devices.clone();
        tokio::spawn(async move {
            while let Ok(device) = daemon.receive_device_added().await {
                let Ok(mut devices) = devices.lock() else {
                    return;
                };
                devices.push(device);
            }
        });

        let daemon = self.daemon.clone();
        let devices = self.devices.clone();
        tokio::spawn(async move {
            while let Ok(device) = daemon.receive_device_removed().await {
                let Ok(mut devices) = devices.lock() else {
                    return;
                };
                if let Some(index) = devices.iter().position(|d| d.id == device.id) {
                    devices.remove(index);
                }
            }
        });

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
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
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        let sessions = &self.session_manager.lock().unwrap().sessions;
        let mut items = sessions
            .iter()
            .enumerate()
            .map(|(index, _)| {
                ListItem::new(format!("App {}", index + 1))
                    .style(Style::default().fg(enabled_color))
            })
            .collect::<Vec<_>>();
        items.push(ListItem::new("▶ Run new app").style(Style::default().fg(default_color)));

        let list = List::new(items)
            .block(block)
            .fg(Color::White)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        f.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }
}
