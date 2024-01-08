// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper for a single bulk tag configuration status response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigResponse {
    /// The status of a request to bulk configure metric tags.
    /// It contains the fields from the original request for reference.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::MetricBulkTagConfigStatus>>,
}

impl MetricBulkTagConfigResponse {
    pub fn new() -> MetricBulkTagConfigResponse {
        MetricBulkTagConfigResponse { data: None }
    }
}
impl Default for MetricBulkTagConfigResponse {
    fn default() -> Self {
        Self::new()
    }
}
