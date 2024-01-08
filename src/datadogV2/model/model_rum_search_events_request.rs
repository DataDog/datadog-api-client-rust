// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a RUM events list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMSearchEventsRequest {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::RUMQueryFilter>>,
    /// Global query options that are used during the query.
    /// Note: Only supply timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::RUMQueryOptions>>,
    /// Paging attributes for listing events.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::RUMQueryPageOptions>>,
    /// Sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::RUMSort>,
}

impl RUMSearchEventsRequest {
    pub fn new() -> RUMSearchEventsRequest {
        RUMSearchEventsRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }
}
impl Default for RUMSearchEventsRequest {
    fn default() -> Self {
        Self::new()
    }
}
