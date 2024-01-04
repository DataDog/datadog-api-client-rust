// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// All the available log-based metric objects.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricsResponse {
    /// A list of log-based metric objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::LogsMetricResponseData>>,
}

impl LogsMetricsResponse {
    pub fn new() -> LogsMetricsResponse {
        LogsMetricsResponse { data: None }
    }
}
