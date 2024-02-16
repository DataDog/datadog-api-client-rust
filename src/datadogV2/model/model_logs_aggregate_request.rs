// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object sent with the request to retrieve a list of logs from your organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateRequest {
    /// The list of metrics or timeseries to compute for the retrieved buckets.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV2::model::LogsCompute>>,
    /// The search and filter query settings
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::LogsQueryFilter>,
    /// The rules for the group by
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::LogsGroupBy>>,
    /// Global query options that are used during the query.
    /// Note: you should supply either timezone or time offset, but not both. Otherwise, the query will fail.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::LogsQueryOptions>,
    /// Paging settings
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::LogsAggregateRequestPage>,
}

impl LogsAggregateRequest {
    pub fn new() -> LogsAggregateRequest {
        LogsAggregateRequest {
            compute: None,
            filter: None,
            group_by: None,
            options: None,
            page: None,
        }
    }

    pub fn compute(&mut self, value: Vec<crate::datadogV2::model::LogsCompute>) -> &mut Self {
        self.compute = Some(value);
        self
    }

    pub fn filter(&mut self, value: crate::datadogV2::model::LogsQueryFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(&mut self, value: Vec<crate::datadogV2::model::LogsGroupBy>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn options(&mut self, value: crate::datadogV2::model::LogsQueryOptions) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn page(&mut self, value: crate::datadogV2::model::LogsAggregateRequestPage) -> &mut Self {
        self.page = Some(value);
        self
    }
}

impl Default for LogsAggregateRequest {
    fn default() -> Self {
        Self::new()
    }
}
