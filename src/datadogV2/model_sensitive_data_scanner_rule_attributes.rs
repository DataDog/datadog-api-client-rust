// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleAttributes {
    /// Description of the rule.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Attributes excluded from the scan. If namespaces is provided, it has to be a sub-path of the namespaces array.
    #[serde(rename = "excluded_namespaces", skip_serializing_if = "Option::is_none")]
    pub excluded_namespaces: Vec<String>,
    /// Whether or not the rule is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Attributes included in the scan. If namespaces is empty or missing, all attributes except excluded_namespaces are scanned.
If both are missing the whole event is scanned.
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Vec<String>,
    /// Not included if there is a relationship to a standard pattern.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: String,
    /// List of tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Object describing how the scanned event will be replaced.
    #[serde(rename = "text_replacement", skip_serializing_if = "Option::is_none")]
    pub text_replacement: SensitiveDataScannerTextReplacement,
}

