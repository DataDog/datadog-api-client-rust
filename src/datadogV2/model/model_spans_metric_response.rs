// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The span-based metric object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricResponse {
    /// The span-based metric properties.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SpansMetricResponseData>,
}

impl SpansMetricResponse {
    pub fn new() -> SpansMetricResponse {
        SpansMetricResponse { data: None }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV2::model::SpansMetricResponseData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for SpansMetricResponse {
    fn default() -> Self {
        Self::new()
    }
}
