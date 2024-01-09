use std::sync::{Arc, Mutex};

use ratatui::{prelude::*, widgets::*};

use crate::{
    daemon::{flutter::FlutterDaemon, io::device::Device},
    tui::Frame,
};
use color_eyre::eyre::Result;

use super::Component;

pub struct DevicesComponent {
    daemon: Arc<FlutterDaemon>,

    devices: Arc<Mutex<Vec<Device>>>,
}

impl DevicesComponent {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self {
            daemon,
            devices: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Component for DevicesComponent {
    fn init(&mut self, area: Rect) -> Result<()> {
        let daemon = self.daemon.clone();
        tokio::spawn(async move {
            let _ = daemon.receive_daemon_connected().await;
            daemon.enable_device().await.unwrap();
        });

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
        let Ok(devices) = self.devices.lock() else {
            return Ok(());
        };

        let block = Block::default().title("Devices").borders(Borders::ALL);
        let items: Vec<ListItem> = devices
            .iter()
            .map(|d| ListItem::new(format!("{} ({})", d.name, d.platform)))
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
