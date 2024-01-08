use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

use super::Component;

pub struct ProjectComponent {
    daemon: Arc<FlutterDaemon>,
}

impl ProjectComponent {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self { daemon }
    }
}

impl Component for ProjectComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default().title("Project").borders(Borders::ALL);
        let text = Paragraph::new("lazyflutter").block(block);
        f.render_widget(text, area);
        Ok(())
    }
}
