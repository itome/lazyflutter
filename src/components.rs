use std::sync::Arc;

use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use crate::{
    action::Action,
    config::Config,
    store::{state::State, Store},
    tui::{Event, Frame},
};

pub mod apps;
pub mod devices;
pub mod devtools;
pub mod home;
pub mod project;

/// `Component` is a trait that represents a visual and interactive element of the user interface.
/// Implementors of this trait can be registered with the main application loop and will be able to receive events,
/// update state, and be rendered on the screen.
pub trait Component {
    /// Register a configuration handler that provides configuration settings if necessary.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    #[allow(unused_variables)]
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        Ok(())
    }
    /// Initialize the component with a specified area if necessary.
    ///
    /// # Arguments
    ///
    /// * `area` - Rectangular area to initialize the component within.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn init(&mut self, area: Rect, store: Arc<Mutex<Store>>) -> Result<()> {
        Ok(())
    }
    /// Handle incoming events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `event` - An optional event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    fn handle_events(
        &mut self,
        event: Option<Event>,
        store: Arc<Mutex<Store>>,
    ) -> Result<Option<Action>> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event, store)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event, store)?,
            _ => None,
        };
        Ok(r)
    }
    /// Handle key events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `key` - A key event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_key_events(
        &mut self,
        key: KeyEvent,
        store: Arc<Mutex<Store>>,
    ) -> Result<Option<Action>> {
        Ok(None)
    }
    /// Handle mouse events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `mouse` - A mouse event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_mouse_events(
        &mut self,
        mouse: MouseEvent,
        store: Arc<Mutex<Store>>,
    ) -> Result<Option<Action>> {
        Ok(None)
    }
    /// Render the component on the screen. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `f` - A frame used for rendering.
    /// * `area` - The area in which the component should be drawn.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<()>;
}
