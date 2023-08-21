// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventResponseAttributes {
    /// Object description of attributes from your event.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: EventAttributes,
    /// The message of the event.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// An array of tags associated with the event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The timestamp of the event.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: String,
}

