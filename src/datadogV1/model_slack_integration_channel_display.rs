// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationChannelDisplay {
    /// Show the main body of the alert event.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: bool,
    /// Show the list of @-handles in the alert event.
    #[serde(rename = "notified", skip_serializing_if = "Option::is_none")]
    pub notified: bool,
    /// Show the alert event's snapshot image.
    #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
    pub snapshot: bool,
    /// Show the scopes on which the monitor alerted.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: bool,
}

