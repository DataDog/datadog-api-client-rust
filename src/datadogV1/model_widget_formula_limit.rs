// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFormulaLimit {
    /// Number of results to return.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i64,
    /// Direction of sort.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: QuerySortOrder,
}

