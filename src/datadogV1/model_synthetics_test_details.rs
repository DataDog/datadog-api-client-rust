// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestDetails {
    /// Configuration object for a Synthetic test.
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: SyntheticsTestConfig,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Creator,
    /// Array of locations used to run the test.
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Vec<String>,
    /// Notification message associated with the test.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id", skip_serializing_if = "Option::is_none")]
    pub monitor_id: i64,
    /// Name of the test.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Object describing the extra options for a Synthetic test.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: SyntheticsTestOptions,
    /// The test public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// Define whether you want to start (`live`) or pause (`paused`) a
Synthetic test.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: SyntheticsTestPauseStatus,
    /// For browser test, the steps of the test.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Vec<SyntheticsStep>,
    /// The subtype of the Synthetic API test, `http`, `ssl`, `tcp`,
`dns`, `icmp`, `udp`, `websocket`, `grpc` or `multi`.
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: SyntheticsTestDetailsSubType,
    /// Array of tags attached to the test.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Type of the Synthetic test, either `api` or `browser`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsTestDetailsType,
}

