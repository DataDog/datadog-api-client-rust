// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddSignalToIncidentRequest {
    /// Whether to post the signal on the incident timeline.
    #[serde(rename = "add_to_signal_timeline", skip_serializing_if = "Option::is_none")]
    pub add_to_signal_timeline: bool,
    /// Public ID attribute of the incident to which the signal will be added.
    #[serde(rename = "incident_id", skip_serializing_if = "Option::is_none")]
    pub incident_id: i64,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i64,
}

