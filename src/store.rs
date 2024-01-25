use std::sync::Arc;

use tokio::sync::{mpsc, Mutex, Notify, RwLock};

use self::state::State;

pub mod action;
pub mod reducer;
pub mod state;

pub struct Store {
    pub state: Arc<Mutex<State>>,
    pub action_tx: mpsc::Sender<action::Action>,
    cancel_notifier: Arc<Notify>,
}

impl Store {
    pub fn new() -> Self {
        let cancel_notifier = Arc::new(Notify::new());
        let (action_tx, mut action_rx) = mpsc::channel::<action::Action>(100);
        let state = Arc::new(Mutex::new(State::default()));

        let _state = state.clone();
        let _cancel_notifier = cancel_notifier.clone();
        tokio::spawn(async move {
            loop {
                let _state = _state.clone();
                tokio::select! {
                    _ = _cancel_notifier.notified() => {}
                    action = action_rx.recv() => {
                        if let Some(action) = action {
                            Self::handle_action(_state, action).await;
                        }
                    }
                }
            }
        });

        Self {
            state,
            action_tx,
            cancel_notifier,
        }
    }

    pub async fn dispatch(&mut self, action: action::Action) {
        self.action_tx.send(action).await.unwrap();
    }

    pub async fn get_state(&self) -> State {
        self.state.lock().await.clone()
    }

    async fn handle_action(state: Arc<Mutex<State>>, action: action::Action) {
        let mut state = state.lock().await;
        *state = reducer::reducer(state.clone(), action);
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        self.cancel_notifier.notify_waiters();
    }
}
