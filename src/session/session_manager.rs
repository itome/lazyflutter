use std::{collections::HashMap, sync::Arc};

use super::session::Session;
use crate::daemon::flutter::FlutterDaemon;
use color_eyre::eyre::Result;

pub struct SessionManager {
    sessions: Vec<Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self { sessions: vec![] }
    }

    pub fn run_new_app(&mut self, path: &str) -> Result<()> {
        let app = Session::new(None, None);
        self.sessions.push(app);
        Ok(())
    }
}
