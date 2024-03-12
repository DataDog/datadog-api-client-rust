// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object sent with the request to retrieve aggregation buckets of pipeline events from your organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelinesAggregateRequest {
    /// The list of metrics or timeseries to compute for the retrieved buckets.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV2::model::CIAppCompute>>,
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::CIAppPipelinesQueryFilter>,
    /// The rules for the group-by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::CIAppPipelinesGroupBy>>,
    /// Global query options that are used during the query.
    /// Only supply timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::CIAppQueryOptions>,
}

impl CIAppPipelinesAggregateRequest {
    pub fn new() -> CIAppPipelinesAggregateRequest {
        CIAppPipelinesAggregateRequest {
            compute: None,
            filter: None,
            group_by: None,
            options: None,
        }
    }

    pub fn compute(mut self, value: Vec<crate::datadogV2::model::CIAppCompute>) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::CIAppPipelinesQueryFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV2::model::CIAppPipelinesGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::CIAppQueryOptions) -> Self {
        self.options = Some(value);
        self
    }
}

impl Default for CIAppPipelinesAggregateRequest {
    fn default() -> Self {
        Self::new()
    }
}
