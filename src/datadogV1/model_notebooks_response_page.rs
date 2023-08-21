// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponsePage {
    /// The total number of notebooks that would be returned if the request was not filtered by `start` and `count` parameters.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: i64,
    /// The total number of notebooks returned.
    #[serde(rename = "total_filtered_count", skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: i64,
}

