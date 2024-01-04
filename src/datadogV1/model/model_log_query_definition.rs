// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The log query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinition {
    /// Define computation for a log query.
    #[serde(rename = "compute")]
    pub compute: Option<Box<crate::datadogV1::model::LogsQueryCompute>>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::LogQueryDefinitionGroupBy>>,
    /// A coma separated-list of index names. Use "*" query all indexes at once. [Multiple Indexes](https://docs.datadoghq.com/logs/indexes/#multiple-indexes)
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// This field is mutually exclusive with `compute`.
    #[serde(rename = "multi_compute")]
    pub multi_compute: Option<Vec<crate::datadogV1::model::LogsQueryCompute>>,
    /// The query being made on the logs.
    #[serde(rename = "search")]
    pub search: Option<Box<crate::datadogV1::model::LogQueryDefinitionSearch>>,
}

impl LogQueryDefinition {
    pub fn new() -> LogQueryDefinition {
        LogQueryDefinition {
            compute: None,
            group_by: None,
            index: None,
            multi_compute: None,
            search: None,
        }
    }
}
