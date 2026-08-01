use std::sync::{Arc, Mutex};

use crate::models::Question;

#[derive(Clone)]
pub struct AppState {
    pub questions: Arc<Mutex<Vec<Question>>>
}

