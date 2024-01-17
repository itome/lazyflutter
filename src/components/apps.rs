use std::sync::{Arc, Mutex};

use ratatui::{prelude::*, widgets::*};

use crate::{
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
}

impl AppsComponent {
    pub fn new(daemon: Arc<FlutterDaemon>, session_manager: Arc<Mutex<SessionManager>>) -> Self {
        Self {
            session_manager,
            daemon,
            devices: Arc::new(Mutex::new(vec![])),
            is_selected: false,
        }
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}

impl Component for AppsComponent {
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
        let block = Block::default()
            .title("Apps")
            .borders(Borders::ALL)
            .border_style(if self.is_selected {
                Style::default()
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let items = ["Run new app"];
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        f.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }
}
