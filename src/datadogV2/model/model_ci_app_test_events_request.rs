// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a tests search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppTestEventsRequest {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::CIAppTestsQueryFilter>>,
    /// Global query options that are used during the query.
    /// Only supply timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::CIAppQueryOptions>>,
    /// Paging attributes for listing events.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::CIAppQueryPageOptions>>,
    /// Sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::CIAppSort>,
}

impl CIAppTestEventsRequest {
    pub fn new() -> CIAppTestEventsRequest {
        CIAppTestEventsRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }
}
impl Default for CIAppTestEventsRequest {
    fn default() -> Self {
        Self::new()
    }
}
