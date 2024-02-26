// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object that includes metric cardinality estimates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricEstimateResponse {
    /// Object for a metric cardinality estimate.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::MetricEstimate>,
}

impl MetricEstimateResponse {
    pub fn new() -> MetricEstimateResponse {
        MetricEstimateResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::MetricEstimate) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for MetricEstimateResponse {
    fn default() -> Self {
        Self::new()
    }
}
