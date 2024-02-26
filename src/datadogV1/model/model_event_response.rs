// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing an event response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventResponse {
    /// Object representing an event.
    #[serde(rename = "event")]
    pub event: Option<crate::datadogV1::model::Event>,
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

    pub fn event(&mut self, value: crate::datadogV1::model::Event) -> &mut Self {
        self.event = Some(value);
        self
    }

    pub fn status(&mut self, value: String) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for EventResponse {
    fn default() -> Self {
        Self::new()
    }
}
