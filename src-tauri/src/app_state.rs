//! Shared application runtime state.

pub struct AppState {
    pub initialized: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self { initialized: false }
    }
}
