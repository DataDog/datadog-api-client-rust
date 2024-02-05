// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a logs list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListRequest {
    /// The search and filter query settings
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::LogsQueryFilter>,
    /// Global query options that are used during the query.
    /// Note: you should supply either timezone or time offset, but not both. Otherwise, the query will fail.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::LogsQueryOptions>,
    /// Paging attributes for listing logs.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::LogsListRequestPage>,
    /// Sort parameters when querying logs.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::LogsSort>,
}

impl LogsListRequest {
    pub fn new() -> LogsListRequest {
        LogsListRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }

    pub fn filter(&mut self, value: crate::datadogV2::model::LogsQueryFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn options(&mut self, value: crate::datadogV2::model::LogsQueryOptions) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn page(&mut self, value: crate::datadogV2::model::LogsListRequestPage) -> &mut Self {
        self.page = Some(value);
        self
    }

    pub fn sort(&mut self, value: crate::datadogV2::model::LogsSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
}

impl Default for LogsListRequest {
    fn default() -> Self {
        Self::new()
    }
}
