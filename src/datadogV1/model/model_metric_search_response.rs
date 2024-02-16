// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the list of metrics matching the search query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSearchResponse {
    /// Search result.
    #[serde(rename = "results")]
    pub results: Option<crate::datadogV1::model::MetricSearchResponseResults>,
}

impl MetricSearchResponse {
    pub fn new() -> MetricSearchResponse {
        MetricSearchResponse { results: None }
    }

    pub fn results(
        &mut self,
        value: crate::datadogV1::model::MetricSearchResponseResults,
    ) -> &mut Self {
        self.results = Some(value);
        self
    }
}

impl Default for MetricSearchResponse {
    fn default() -> Self {
        Self::new()
    }
}
