// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object for the pipeline events aggregate API endpoint.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelinesAnalyticsAggregateResponse {
    /// The query results.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::CIAppPipelinesAggregationBucketsResponse>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::CIAppResponseLinks>>,
    /// The metadata associated with a request.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::CIAppResponseMetadata>>,
}

impl CIAppPipelinesAnalyticsAggregateResponse {
    pub fn new() -> CIAppPipelinesAnalyticsAggregateResponse {
        CIAppPipelinesAnalyticsAggregateResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}
impl Default for CIAppPipelinesAnalyticsAggregateResponse {
    fn default() -> Self {
        Self::new()
    }
}
