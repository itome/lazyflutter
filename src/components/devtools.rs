use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};

use crate::{daemon::flutter::FlutterDaemon, store::state::State, tui::Frame};
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
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, _: &State) -> Result<()> {
        let block = Block::default().title("DevTools").borders(Borders::ALL);
        f.render_widget(block, area);
        Ok(())
    }
}
