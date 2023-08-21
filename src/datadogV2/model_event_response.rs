// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventResponse {
    /// The object description of an event response attribute.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: EventResponseAttributes,
    /// the unique ID of the event.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Type of the event.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: EventType,
}

