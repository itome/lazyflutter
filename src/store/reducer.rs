use crate::store::action::Action;

use super::state::State;

pub fn reducer(state: State, action: Action) -> State {
    match action {
        Action::AddDevice { device } => State {
            devices: [state.devices, vec![device]].concat(),
            ..state
        },
        Action::RemoveDevice { device } => State {
            devices: state.devices.into_iter().filter(|d| d != &device).collect(),
            ..state
        },
    }
}
