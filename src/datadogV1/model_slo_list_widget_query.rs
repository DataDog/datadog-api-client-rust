// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListWidgetQuery {
    /// Maximum number of results to display in the table.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// Widget query.
    #[serde(rename = "query_string", skip_serializing_if = "Option::is_none")]
    pub query_string: String,
    /// Options for sorting results.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Vec<WidgetFieldSort>,
}

