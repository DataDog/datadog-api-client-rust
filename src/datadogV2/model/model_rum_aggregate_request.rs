// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object sent with the request to retrieve aggregation buckets of RUM events from your organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMAggregateRequest {
    /// The list of metrics or timeseries to compute for the retrieved buckets.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV2::model::RUMCompute>>,
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::RUMQueryFilter>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::RUMGroupBy>>,
    /// Global query options that are used during the query.
    /// Note: Only supply timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::RUMQueryOptions>,
    /// Paging attributes for listing events.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::RUMQueryPageOptions>,
}

impl RUMAggregateRequest {
    pub fn new() -> RUMAggregateRequest {
        RUMAggregateRequest {
            compute: None,
            filter: None,
            group_by: None,
            options: None,
            page: None,
        }
    }

    pub fn with_compute(&mut self, value: Vec<crate::datadogV2::model::RUMCompute>) -> &mut Self {
        self.compute = Some(value);
        self
    }

    pub fn with_filter(&mut self, value: crate::datadogV2::model::RUMQueryFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn with_group_by(&mut self, value: Vec<crate::datadogV2::model::RUMGroupBy>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn with_options(&mut self, value: crate::datadogV2::model::RUMQueryOptions) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn with_page(&mut self, value: crate::datadogV2::model::RUMQueryPageOptions) -> &mut Self {
        self.page = Some(value);
        self
    }
}
impl Default for RUMAggregateRequest {
    fn default() -> Self {
        Self::new()
    }
}
