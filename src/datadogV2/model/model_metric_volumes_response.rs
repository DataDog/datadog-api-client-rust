// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object which includes a single metric's volume.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricVolumesResponse {
    /// Possible response objects for a metric's volume.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::MetricVolumes>>,
}

impl MetricVolumesResponse {
    pub fn new() -> MetricVolumesResponse {
        MetricVolumesResponse { data: None }
    }
}
impl Default for MetricVolumesResponse {
    fn default() -> Self {
        Self::new()
    }
}
