mod config;

use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

pub use self::config::*;
use crate::Error;

#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Result<Self, Error> {
        Ok(Self { config })
    }
}

#[derive(Debug, Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl AppState {
    pub fn new(app: Arc<App>) -> Self {
        Self(app)
    }
}

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
