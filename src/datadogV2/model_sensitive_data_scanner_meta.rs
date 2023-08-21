// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerMeta {
    /// Maximum number of scanning rules allowed for the org.
    #[serde(rename = "count_limit", skip_serializing_if = "Option::is_none")]
    pub count_limit: i64,
    /// Maximum number of scanning groups allowed for the org.
    #[serde(rename = "group_count_limit", skip_serializing_if = "Option::is_none")]
    pub group_count_limit: i64,
    /// Whether or not scanned events are highlighted in Logs or RUM for the org.
    #[serde(rename = "has_highlight_enabled", skip_serializing_if = "Option::is_none")]
    pub has_highlight_enabled: bool,
    /// Whether or not scanned events have multi-pass enabled.
    #[serde(rename = "has_multi_pass_enabled", skip_serializing_if = "Option::is_none")]
    pub has_multi_pass_enabled: bool,
    /// Whether or not the org is compliant to the payment card industry standard.
    #[serde(rename = "is_pci_compliant", skip_serializing_if = "Option::is_none")]
    pub is_pci_compliant: bool,
    /// Version of the API.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i64,
}

