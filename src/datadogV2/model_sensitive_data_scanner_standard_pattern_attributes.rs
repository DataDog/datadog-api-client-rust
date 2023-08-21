// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerStandardPatternAttributes {
    /// Description of the standard pattern.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Name of the standard pattern.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Regex to match.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: String,
    /// List of tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

