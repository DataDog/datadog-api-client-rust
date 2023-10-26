// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricsResponse {
    /// A list of span-based metric objects.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::datadogV2::model::SpansMetricResponseData>>,
}

impl SpansMetricsResponse {
    /// All the available span-based metric objects.
    pub fn new() -> SpansMetricsResponse {
        SpansMetricsResponse { data: None }
    }
}
