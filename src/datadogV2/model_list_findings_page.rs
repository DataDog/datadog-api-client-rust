// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFindingsPage {
    /// The cursor used to paginate requests.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: String,
    /// The total count of findings after the filter has been applied.
    #[serde(rename = "total_filtered_count", skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: i64,
}

