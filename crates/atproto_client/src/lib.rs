// SPDX-License-Identifier: MIT OR Apache-2.0
//! ATproto client library for querying content moderation labels

#![forbid(unsafe_code)]

mod types;
mod error;
mod resolver;
mod labeler;
mod posts;
mod auth;

// Public API exports (used by web UI)
pub use types::{Did, Handle, Label, LabelCollection, LabelCategory, AtRecord};
pub use labeler::LabelerClient;
pub use posts::PostClient;
pub use resolver::{resolve_handle, resolve_did};
pub use auth::create_session;

// Internal types (not exported, only used internally)
pub(crate) use types::{LabelsResponse, ListRecordsResponse};
pub(crate) use error::{Error, Result};


