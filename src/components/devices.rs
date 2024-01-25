use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};
use tokio::sync::Mutex;

use crate::{
    daemon::{flutter::FlutterDaemon, io::device::Device},
    store::{action::Action, state::State, Store},
    tui::Frame,
};
use color_eyre::eyre::Result;

use super::Component;

pub struct DevicesComponent {
    daemon: Arc<FlutterDaemon>,
    is_selected: bool,
}

impl DevicesComponent {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self {
            daemon,
            is_selected: false,
        }
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}

impl Component for DevicesComponent {
    fn init(&mut self, area: Rect, store: Arc<Mutex<Store>>) -> Result<()> {
        let daemon = self.daemon.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(device) = daemon.receive_device_added() => {
                        store.lock().await.dispatch(Action::AddDevice { device }).await;
                    },
                    Ok(device) = daemon.receive_device_removed() => {
                        store.lock().await.dispatch(Action::RemoveDevice { device }).await;
                    },
                }
            }
        });
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<()> {
        let devices = state.devices.clone();
        let default_color = if self.is_selected {
            Color::White
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Devices")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        let items: Vec<ListItem> = devices
            .iter()
            .map(|d| {
                ListItem::new(format!("{} ({})", d.name, d.platform))
                    .style(Style::default().fg(default_color))
            })
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .block(block);

        f.render_widget(list, area);
        Ok(())
    }
}
