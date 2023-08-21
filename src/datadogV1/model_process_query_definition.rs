// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessQueryDefinition {
    /// List of processes.
    #[serde(rename = "filter_by", skip_serializing_if = "Option::is_none")]
    pub filter_by: Vec<String>,
    /// Max number of items in the filter list.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// Your chosen metric.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Your chosen search term.
    #[serde(rename = "search_by", skip_serializing_if = "Option::is_none")]
    pub search_by: String,
}

