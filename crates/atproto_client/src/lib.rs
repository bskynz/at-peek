// SPDX-License-Identifier: MIT OR Apache-2.0
//! ATproto client library for querying content moderation labels

#![forbid(unsafe_code)]

mod auth;
mod error;
mod labeler;
mod posts;
mod resolver;
mod types;

// Public API exports (used by web UI)
pub use auth::create_session;
pub use labeler::LabelerClient;
pub use posts::PostClient;
pub use resolver::{resolve_did, resolve_handle};
pub use types::{AtRecord, Did, Handle, Label, LabelCategory, LabelCollection};

// Internal types (not exported, only used internally)
pub(crate) use error::{Error, Result};
pub(crate) use types::{LabelsResponse, ListRecordsResponse};
