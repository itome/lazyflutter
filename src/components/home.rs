use std::{collections::HashMap, sync::Arc, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use super::{
    apps::AppsComponent, devices::DevicesComponent, devtools::DevToolsComponent,
    project::ProjectComponent, Component, Frame,
};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    daemon::flutter::FlutterDaemon,
    session::session_manager::SessionManager,
    store::{state::State, Store},
};

#[derive(PartialEq)]
enum Tab {
    Project,
    Apps,
    Devices,
}

pub struct Home {
    project: ProjectComponent,
    apps: AppsComponent,
    devices: DevicesComponent,
    devtools: DevToolsComponent,

    selected_tab: Tab,
}

impl Home {
    pub fn new(daemon: Arc<FlutterDaemon>, session_manager: Arc<Mutex<SessionManager>>) -> Self {
        let devices = DevicesComponent::new(daemon.clone());
        let project = ProjectComponent::new(daemon.clone());
        let mut apps = AppsComponent::new(daemon.clone(), session_manager.clone());
        apps.set_selected(true);
        let devtools = DevToolsComponent::new(daemon.clone());
        Self {
            project,
            devices,
            apps,
            devtools,
            selected_tab: Tab::Apps,
        }
    }
}

impl Component for Home {
    fn init(&mut self, area: Rect, store: Arc<Mutex<Store>>) -> Result<()> {
        self.project.init(area, store.clone())?;
        self.apps.init(area, store.clone())?;
        self.devices.init(area, store.clone())?;
        self.devtools.init(area, store.clone())?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.project.register_config_handler(config.clone())?;
        self.apps.register_config_handler(config.clone())?;
        self.devices.register_config_handler(config.clone())?;
        self.devtools.register_config_handler(config.clone())?;
        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key: KeyEvent,
        store: Arc<Mutex<Store>>,
    ) -> Result<Option<Action>> {
        if key.code == KeyCode::Left {
            self.selected_tab = match self.selected_tab {
                Tab::Project => Tab::Devices,
                Tab::Apps => Tab::Project,
                Tab::Devices => Tab::Apps,
            };
            self.project.set_selected(self.selected_tab == Tab::Project);
            self.apps.set_selected(self.selected_tab == Tab::Apps);
            self.devices.set_selected(self.selected_tab == Tab::Devices);
            return Ok(None);
        } else if key.code == KeyCode::Right {
            self.selected_tab = match self.selected_tab {
                Tab::Project => Tab::Apps,
                Tab::Apps => Tab::Devices,
                Tab::Devices => Tab::Project,
            };
            self.project.set_selected(self.selected_tab == Tab::Project);
            self.apps.set_selected(self.selected_tab == Tab::Apps);
            self.devices.set_selected(self.selected_tab == Tab::Devices);
            return Ok(None);
        }
        if let Tab::Project = self.selected_tab {
            return self.project.handle_key_events(key, store);
        }
        if let Tab::Apps = self.selected_tab {
            return self.apps.handle_key_events(key, store);
        }
        if let Tab::Devices = self.selected_tab {
            return self.devices.handle_key_events(key, store);
        }
        Ok(None)
    }

    fn handle_mouse_events(
        &mut self,
        mouse: MouseEvent,
        store: Arc<Mutex<Store>>,
    ) -> Result<Option<Action>> {
        self.project.handle_mouse_events(mouse, store.clone())?;
        self.apps.handle_mouse_events(mouse, store.clone())?;
        self.devices.handle_mouse_events(mouse, store.clone())?;
        self.devtools.handle_mouse_events(mouse, store.clone())?;
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<()> {
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

        self.project.draw(f, tab_layout[0], state)?;
        self.apps.draw(f, tab_layout[1], state)?;
        self.devices.draw(f, tab_layout[2], state)?;
        self.devtools.draw(f, layout[1], state)?;
        Ok(())
    }
}
