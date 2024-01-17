use std::sync::{Arc, Mutex};

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::{
    daemon::{flutter::FlutterDaemon, io::platform::Platform},
    tui::Frame,
};
use color_eyre::eyre::Result;

use super::Component;

pub struct ProjectComponent {
    daemon: Arc<FlutterDaemon>,
    platforms: Arc<Mutex<Vec<String>>>,
}

impl ProjectComponent {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self {
            daemon,
            platforms: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Component for ProjectComponent {
    fn init(&mut self, area: Rect) -> Result<()> {
        let daemon = self.daemon.clone();
        let platforms = self.platforms.clone();
        tokio::spawn(async move {
            while let Ok(mut _platforms) = daemon
                .get_supported_platforms(String::from(
                    "/Path/Your/Flutter/Projects/",
                ))
                .await
            {
                match platforms.lock() {
                    Ok(mut platforms) => {
                        platforms.clear();
                        platforms.append(&mut _platforms);
                    }
                    Err(_) => return,
                }
            }
        });
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let Ok(platforms) = self.platforms.lock() else {
            return Ok(());
        };
        let block = Block::default().title("Project").borders(Borders::ALL);
        let ps: Vec<String> = platforms.iter().map(|p| p.clone()).collect();
        let mut items: Vec<ListItem> = vec![];
        items.push(ListItem::new(format!("lazyflutter ({})", ps.join(","))));

        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .block(block);

        f.render_widget(list, area);
        Ok(())
    }
}
