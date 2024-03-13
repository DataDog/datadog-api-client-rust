// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata associated with a request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Event ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The event name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Event source ID.
    #[serde(rename = "source_id")]
    pub source_id: Option<i64>,
    /// Event type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl Event {
    pub fn new() -> Event {
        Event {
            id: None,
            name: None,
            source_id: None,
            type_: None,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn source_id(mut self, value: i64) -> Self {
        self.source_id = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}
