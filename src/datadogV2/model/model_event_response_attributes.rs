// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object description of an event response attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventResponseAttributes {
    /// Object description of attributes from your event.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::EventAttributes>,
    /// The message of the event.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// An array of tags associated with the event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The timestamp of the event.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
}

impl EventResponseAttributes {
    pub fn new() -> EventResponseAttributes {
        EventResponseAttributes {
            attributes: None,
            message: None,
            tags: None,
            timestamp: None,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::EventAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
        self
    }
}

impl Default for EventResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
