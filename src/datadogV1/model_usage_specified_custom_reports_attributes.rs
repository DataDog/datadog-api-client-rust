// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsAttributes {
    /// The date the specified custom report was computed.
    #[serde(rename = "computed_on", skip_serializing_if = "Option::is_none")]
    pub computed_on: String,
    /// The ending date of specified custom report.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: String,
    /// A downloadable file for the specified custom reporting file.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: String,
    /// size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: i64,
    /// The starting date of specified custom report.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: String,
    /// A list of tags to apply to specified custom reports.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

