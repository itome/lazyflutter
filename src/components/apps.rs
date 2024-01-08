use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};

use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

use super::Component;

pub struct AppsComponent {
    daemon: Arc<FlutterDaemon>,
}

impl AppsComponent {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self { daemon }
    }
}

impl Component for AppsComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default().title("Apps").borders(Borders::ALL);
        f.render_widget(block, area);
        Ok(())
    }
}
