// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric's actively queried tags and aggregations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSuggestedTagsAttributes {
    /// List of aggregation combinations that have been actively queried.
    #[serde(rename = "active_aggregations")]
    pub active_aggregations: Option<Vec<crate::datadogV2::model::MetricCustomAggregation>>,
    /// List of tag keys that have been actively queried.
    #[serde(rename = "active_tags")]
    pub active_tags: Option<Vec<String>>,
}

impl MetricSuggestedTagsAttributes {
    pub fn new() -> MetricSuggestedTagsAttributes {
        MetricSuggestedTagsAttributes {
            active_aggregations: None,
            active_tags: None,
        }
    }

    pub fn active_aggregations(
        &mut self,
        value: Vec<crate::datadogV2::model::MetricCustomAggregation>,
    ) -> &mut Self {
        self.active_aggregations = Some(value);
        self
    }

    pub fn active_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.active_tags = Some(value);
        self
    }
}

impl Default for MetricSuggestedTagsAttributes {
    fn default() -> Self {
        Self::new()
    }
}
