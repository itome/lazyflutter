use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};

use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

use super::Component;

pub struct DevToolsComponent {
    daemon: Arc<FlutterDaemon>,
}

impl DevToolsComponent {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self { daemon }
    }
}

impl Component for DevToolsComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default().title("DevTools").borders(Borders::ALL);
        f.render_widget(block, area);
        Ok(())
    }
}
