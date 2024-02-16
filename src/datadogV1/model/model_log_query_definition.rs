// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The log query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinition {
    /// Define computation for a log query.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1::model::LogsQueryCompute>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::LogQueryDefinitionGroupBy>>,
    /// A coma separated-list of index names. Use "*" query all indexes at once. [Multiple Indexes](<https://docs.datadoghq.com/logs/indexes/#multiple-indexes>)
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// This field is mutually exclusive with `compute`.
    #[serde(rename = "multi_compute")]
    pub multi_compute: Option<Vec<crate::datadogV1::model::LogsQueryCompute>>,
    /// The query being made on the logs.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1::model::LogQueryDefinitionSearch>,
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

    pub fn compute(&mut self, value: crate::datadogV1::model::LogsQueryCompute) -> &mut Self {
        self.compute = Some(value);
        self
    }

    pub fn group_by(
        &mut self,
        value: Vec<crate::datadogV1::model::LogQueryDefinitionGroupBy>,
    ) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn index(&mut self, value: String) -> &mut Self {
        self.index = Some(value);
        self
    }

    pub fn multi_compute(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsQueryCompute>,
    ) -> &mut Self {
        self.multi_compute = Some(value);
        self
    }

    pub fn search(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinitionSearch,
    ) -> &mut Self {
        self.search = Some(value);
        self
    }
}

impl Default for LogQueryDefinition {
    fn default() -> Self {
        Self::new()
    }
}
