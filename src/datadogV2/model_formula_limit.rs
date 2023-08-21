// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaLimit {
    /// The number of results to which to limit.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i32,
    /// Direction of sort.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: QuerySortOrder,
}

