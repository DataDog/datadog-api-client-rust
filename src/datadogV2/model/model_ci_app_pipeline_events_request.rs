// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a pipelines search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventsRequest {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::CIAppPipelinesQueryFilter>,
    /// Global query options that are used during the query.
    /// Only supply timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::CIAppQueryOptions>,
    /// Paging attributes for listing events.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::CIAppQueryPageOptions>,
    /// Sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::CIAppSort>,
}

impl CIAppPipelineEventsRequest {
    pub fn new() -> CIAppPipelineEventsRequest {
        CIAppPipelineEventsRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }

    pub fn with_filter(
        &mut self,
        value: crate::datadogV2::model::CIAppPipelinesQueryFilter,
    ) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn with_options(&mut self, value: crate::datadogV2::model::CIAppQueryOptions) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn with_page(
        &mut self,
        value: crate::datadogV2::model::CIAppQueryPageOptions,
    ) -> &mut Self {
        self.page = Some(value);
        self
    }

    pub fn with_sort(&mut self, value: crate::datadogV2::model::CIAppSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
}
impl Default for CIAppPipelineEventsRequest {
    fn default() -> Self {
        Self::new()
    }
}
