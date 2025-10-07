// SPDX-License-Identifier: MIT OR Apache-2.0
//! at-peek web UI - Inspect content moderation labels on ATproto

#![forbid(unsafe_code)]

use leptos::*;
use wasm_bindgen::prelude::*;

mod components;
mod state;
mod utils;

use components::App;

#[wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    log::info!("at-peek web UI starting...");
    
    leptos::mount_to_body(App);
}


