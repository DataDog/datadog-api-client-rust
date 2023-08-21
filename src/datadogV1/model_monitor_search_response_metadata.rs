// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResponseMetadata {
    /// The page to start paginating from.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: i64,
    /// The number of pages.
    #[serde(rename = "page_count", skip_serializing_if = "Option::is_none")]
    pub page_count: i64,
    /// The number of monitors to return per page.
    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
    pub per_page: i64,
    /// The total number of monitors.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: i64,
}

