// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Event ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The event name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Event source ID.
    #[serde(rename = "source_id", skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,
    /// Event type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Event {
    /// The metadata associated with a request.
    pub fn new() -> Event {
        Event {
            id: None,
            name: None,
            source_id: None,
            type_: None,
        }
    }
}
