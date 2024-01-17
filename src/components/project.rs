use std::sync::{Arc, Mutex};

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

use super::Component;

pub struct ProjectComponent {
    daemon: Arc<FlutterDaemon>,
    platforms: Arc<Mutex<Vec<String>>>,
    project_root: String,
    is_selected: bool,
}

impl ProjectComponent {
    pub fn new(daemon: Arc<FlutterDaemon>, project_root: String) -> Self {
        Self {
            daemon,
            platforms: Arc::new(Mutex::new(vec![])),
            project_root: project_root,
            is_selected: false,
        }
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}

impl Component for ProjectComponent {
    fn init(&mut self, area: Rect) -> Result<()> {
        let daemon = self.daemon.clone();
        let platforms = self.platforms.clone();
        let project_root = self.project_root.clone();
        tokio::spawn(async move {
            while let Ok(mut _platforms) = daemon.get_supported_platforms(project_root.clone()).await {
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

        let default_color = if self.is_selected {
            Color::White
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Project")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));
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
