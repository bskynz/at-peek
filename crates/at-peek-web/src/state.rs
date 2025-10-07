// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::*;
use atproto_client::LabelCollection;

/// Global application state
#[derive(Clone, Copy)]
pub struct AppState {
    /// Current input (handle, DID, or AT-URI)
    pub subject_input: RwSignal<String>,
    
    /// Fetched labels
    pub labels: RwSignal<Option<LabelCollection>>,
    
    /// Loading state
    pub is_loading: RwSignal<bool>,
    
    /// Error message (if any)
    pub error: RwSignal<Option<String>>,
    
    /// Authentication token
    pub auth_token: RwSignal<Option<String>>,
    
    /// Is user authenticated
    pub is_authenticated: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            subject_input: create_rw_signal(String::new()),
            labels: create_rw_signal(None),
            is_loading: create_rw_signal(false),
            error: create_rw_signal(None),
            auth_token: create_rw_signal(None),
            is_authenticated: create_rw_signal(false),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}


