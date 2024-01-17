// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing an event response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventResponse {
    /// Object representing an event.
    #[serde(rename = "event")]
    pub event: Option<Box<crate::datadogV1::model::Event>>,
    /// A status.
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl EventResponse {
    pub fn new() -> EventResponse {
        EventResponse {
            event: None,
            status: None,
        }
    }
}
impl Default for EventResponse {
    fn default() -> Self {
        Self::new()
    }
}
