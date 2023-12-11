// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object description of an event after being processed and stored by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventResponse {
    /// The object description of an event response attribute.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::EventResponseAttributes>>,
    /// the unique ID of the event.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::EventType>,
}

impl EventResponse {
    pub fn new() -> EventResponse {
        EventResponse {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
