// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyTagPolicy {
    /// The key of the tag.
    #[serde(rename = "tag_key", skip_serializing_if = "Option::is_none")]
    pub tag_key: String,
    /// If a tag key is required for monitor creation.
    #[serde(rename = "tag_key_required", skip_serializing_if = "Option::is_none")]
    pub tag_key_required: bool,
    /// Valid values for the tag.
    #[serde(rename = "valid_tag_values", skip_serializing_if = "Option::is_none")]
    pub valid_tag_values: Vec<String>,
}

