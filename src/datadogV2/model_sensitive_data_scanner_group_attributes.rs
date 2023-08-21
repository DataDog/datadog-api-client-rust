// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupAttributes {
    /// Description of the group.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Filter for the Scanning Group.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: SensitiveDataScannerFilter,
    /// Whether or not the group is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// List of products the scanning group applies.
    #[serde(rename = "product_list", skip_serializing_if = "Option::is_none")]
    pub product_list: Vec<SensitiveDataScannerProduct>,
}

