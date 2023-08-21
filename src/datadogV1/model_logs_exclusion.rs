// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsExclusion {
    /// Exclusion filter is defined by a query, a sampling rule, and a active/inactive toggle.
    #[serde(rename = "filter")]
    pub filter: LogsExclusionFilter,
    /// Whether or not the exclusion filter is active.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the index exclusion filter.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}

