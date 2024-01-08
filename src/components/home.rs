use std::{collections::HashMap, sync::Arc, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    apps::AppsComponent, devices::DevicesComponent, devtools::DevToolsComponent,
    project::ProjectComponent, Component, Frame,
};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    daemon::flutter::FlutterDaemon,
};

pub struct Home {
    project: ProjectComponent,
    apps: AppsComponent,
    devices: DevicesComponent,
    devtools: DevToolsComponent,
}

impl Home {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        let devices = DevicesComponent::new(daemon.clone());
        let project = ProjectComponent::new(daemon.clone());
        let apps = AppsComponent::new(daemon.clone());
        let devtools = DevToolsComponent::new(daemon.clone());
        Self {
            project,
            devices,
            apps,
            devtools,
        }
    }
}

impl Component for Home {
    fn init(&mut self, area: Rect) -> Result<()> {
        self.project.init(area)?;
        self.apps.init(area)?;
        self.devices.init(area)?;
        self.devtools.init(area)?;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.project.register_action_handler(tx.clone())?;
        self.apps.register_action_handler(tx.clone())?;
        self.devices.register_action_handler(tx.clone())?;
        self.devtools.register_action_handler(tx.clone())?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.project.register_config_handler(config.clone())?;
        self.apps.register_config_handler(config.clone())?;
        self.devices.register_config_handler(config.clone())?;
        self.devtools.register_config_handler(config.clone())?;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.project.handle_key_events(key)?;
        self.apps.handle_key_events(key)?;
        self.devices.handle_key_events(key)?;
        self.devtools.handle_key_events(key)?;
        Ok(None)
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        self.project.handle_mouse_events(mouse)?;
        self.apps.handle_mouse_events(mouse)?;
        self.devices.handle_mouse_events(mouse)?;
        self.devtools.handle_mouse_events(mouse)?;
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        self.project.update(action.clone())?;
        self.apps.update(action.clone())?;
        self.devices.update(action.clone())?;
        self.devtools.update(action.clone())?;
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);
        let tab_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(layout[0]);

        self.project.draw(f, tab_layout[0])?;
        self.apps.draw(f, tab_layout[1])?;
        self.devices.draw(f, tab_layout[2])?;
        self.devtools.draw(f, layout[1])?;
        Ok(())
    }
}
