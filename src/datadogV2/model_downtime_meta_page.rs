// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeMetaPage {
    /// Total count of elements matched by the filter.
    #[serde(rename = "total_filtered_count", skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: i64,
}

