use std::{collections::HashMap, sync::Arc};

use super::session::Session;
use crate::daemon::flutter::FlutterDaemon;
use color_eyre::eyre::Result;

pub struct SessionManager {
    pub project_root: Option<String>,
    pub sessions: Vec<Session>,
}

impl SessionManager {
    pub fn new(project_root: Option<String>) -> Self {
        Self {
            sessions: vec![],
            project_root,
        }
    }

    pub fn run_new_app(&mut self) -> Result<()> {
        let app = Session::new(
            self.project_root.as_ref().map(|path| -> &str { &path }),
            None,
        );
        self.sessions.push(app);
        Ok(())
    }
}
