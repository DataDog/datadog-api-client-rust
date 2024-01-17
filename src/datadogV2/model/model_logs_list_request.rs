// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a logs list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListRequest {
    /// The search and filter query settings
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::LogsQueryFilter>>,
    /// Global query options that are used during the query.
    /// Note: you should supply either timezone or time offset, but not both. Otherwise, the query will fail.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::LogsQueryOptions>>,
    /// Paging attributes for listing logs.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::LogsListRequestPage>>,
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
}
impl Default for LogsListRequest {
    fn default() -> Self {
        Self::new()
    }
}
