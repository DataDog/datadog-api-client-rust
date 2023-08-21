// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalAttributes {
    /// A JSON object of attributes in the security signal.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: map[string]interface{},
    /// The message in the security signal defined by the rule that generated the signal.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// An array of tags associated with the security signal.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The timestamp of the security signal.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: String,
}

