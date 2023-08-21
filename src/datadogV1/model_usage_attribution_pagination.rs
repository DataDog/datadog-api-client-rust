// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// Records to be skipped before beginning to return.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// Direction to sort by.
    #[serde(rename = "sort_direction", skip_serializing_if = "Option::is_none")]
    pub sort_direction: String,
    /// Field to sort by.
    #[serde(rename = "sort_name", skip_serializing_if = "Option::is_none")]
    pub sort_name: String,
    /// Total number of records.
    #[serde(rename = "total_number_of_records", skip_serializing_if = "Option::is_none")]
    pub total_number_of_records: i64,
}

