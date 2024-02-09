// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// All the available span-based metric objects.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricsResponse {
    /// A list of span-based metric objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::SpansMetricResponseData>>,
}

impl SpansMetricsResponse {
    pub fn new() -> SpansMetricsResponse {
        SpansMetricsResponse { data: None }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::SpansMetricResponseData>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for SpansMetricsResponse {
    fn default() -> Self {
        Self::new()
    }
}
