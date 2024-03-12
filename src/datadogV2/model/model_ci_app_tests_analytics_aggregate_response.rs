// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object for the test events aggregate API endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppTestsAnalyticsAggregateResponse {
    /// The query results.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::CIAppTestsAggregationBucketsResponse>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::CIAppResponseLinks>,
    /// The metadata associated with a request.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CIAppResponseMetadataWithPagination>,
}

impl CIAppTestsAnalyticsAggregateResponse {
    pub fn new() -> CIAppTestsAnalyticsAggregateResponse {
        CIAppTestsAnalyticsAggregateResponse {
            data: None,
            links: None,
            meta: None,
        }
    }

    pub fn data(
        mut self,
        value: crate::datadogV2::model::CIAppTestsAggregationBucketsResponse,
    ) -> Self {
        self.data = Some(value);
        self
    }

    pub fn links(mut self, value: crate::datadogV2::model::CIAppResponseLinks) -> Self {
        self.links = Some(value);
        self
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::CIAppResponseMetadataWithPagination,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for CIAppTestsAnalyticsAggregateResponse {
    fn default() -> Self {
        Self::new()
    }
}
