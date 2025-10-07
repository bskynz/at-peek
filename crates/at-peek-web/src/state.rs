// SPDX-License-Identifier: MIT OR Apache-2.0

use atproto_client::LabelCollection;
use leptos::prelude::*;

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

    /// Authenticated user's DID
    pub authenticated_user_did: RwSignal<Option<String>>,

    /// Is user authenticated
    pub is_authenticated: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            subject_input: RwSignal::new(String::new()),
            labels: RwSignal::new(None),
            is_loading: RwSignal::new(false),
            error: RwSignal::new(None),
            auth_token: RwSignal::new(None),
            authenticated_user_did: RwSignal::new(None),
            is_authenticated: RwSignal::new(false),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
