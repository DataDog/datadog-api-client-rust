// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// The cursor to get the next results (if any). To make the next request, use the same parameters and add `next_record_id`.
    #[serde(rename = "next_record_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub next_record_id: Option<String>,
    /// Total number of records.
    #[serde(rename = "total_number_of_records", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub total_number_of_records: Option<Int64>,
}

