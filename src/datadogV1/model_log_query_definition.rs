// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinition {
    /// Define computation for a log query.
    #[serde(rename = "compute")]
    pub compute: LogsQueryCompute,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<LogQueryDefinitionGroupBy>,
    /// A coma separated-list of index names. Use "*" query all indexes at once. [Multiple Indexes](https://docs.datadoghq.com/logs/indexes/#multiple-indexes)
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: String,
    /// This field is mutually exclusive with `compute`.
    #[serde(rename = "multi_compute", skip_serializing_if = "Option::is_none")]
    pub multi_compute: Vec<LogsQueryCompute>,
    /// The query being made on the logs.
    #[serde(rename = "search")]
    pub search: LogQueryDefinitionSearch,
}

