// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An event list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventListResponse {
    /// An array of events.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV1::model::Event>>,
    /// A status.
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl EventListResponse {
    pub fn new() -> EventListResponse {
        EventListResponse {
            events: None,
            status: None,
        }
    }

    pub fn events(mut self, value: Vec<crate::datadogV1::model::Event>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for EventListResponse {
    fn default() -> Self {
        Self::new()
    }
}
