// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object that includes a single metric's actively queried tags and aggregations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSuggestedTagsAndAggregationsResponse {
    /// Object for a single metric's actively queried tags and aggregations.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::MetricSuggestedTagsAndAggregations>,
}

impl MetricSuggestedTagsAndAggregationsResponse {
    pub fn new() -> MetricSuggestedTagsAndAggregationsResponse {
        MetricSuggestedTagsAndAggregationsResponse { data: None }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV2::model::MetricSuggestedTagsAndAggregations,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for MetricSuggestedTagsAndAggregationsResponse {
    fn default() -> Self {
        Self::new()
    }
}
