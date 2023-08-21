// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResponseCounts {
    /// Search facets.
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Vec<MonitorSearchCountItem>,
    /// Search facets.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Vec<MonitorSearchCountItem>,
    /// Search facets.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Vec<MonitorSearchCountItem>,
    /// Search facets.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Vec<MonitorSearchCountItem>,
}

