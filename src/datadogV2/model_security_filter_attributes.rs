// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterAttributes {
    /// The list of exclusion filters applied in this security filter.
    #[serde(rename = "exclusion_filters", skip_serializing_if = "Option::is_none")]
    pub exclusion_filters: Vec<SecurityFilterExclusionFilterResponse>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type", skip_serializing_if = "Option::is_none")]
    pub filtered_data_type: SecurityFilterFilteredDataType,
    /// Whether the security filter is the built-in filter.
    #[serde(rename = "is_builtin", skip_serializing_if = "Option::is_none")]
    pub is_builtin: bool,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// The security filter name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The security filter query. Logs accepted by this query will be accepted by this filter.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The version of the security filter.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i32,
}

