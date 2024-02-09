// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object which includes a single metric's tag configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationResponse {
    /// Object for a single metric tag configuration.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::MetricTagConfiguration>,
}

impl MetricTagConfigurationResponse {
    pub fn new() -> MetricTagConfigurationResponse {
        MetricTagConfigurationResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::MetricTagConfiguration) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for MetricTagConfigurationResponse {
    fn default() -> Self {
        Self::new()
    }
}
