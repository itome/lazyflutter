use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

use super::Component;

pub struct ProjectComponent {
    daemon: Arc<FlutterDaemon>,
    is_selected: bool,
}

impl ProjectComponent {
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

impl Component for ProjectComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default()
            .title("Project")
            .borders(Borders::ALL)
            .border_style(if self.is_selected {
                Style::default()
            } else {
                Style::default().fg(Color::DarkGray)
            });
        let text = Paragraph::new("lazyflutter").block(block);
        f.render_widget(text, area);
        Ok(())
    }
}
