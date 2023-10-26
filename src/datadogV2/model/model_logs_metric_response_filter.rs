// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricResponseFilter {
    /// The search query - following the log search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl LogsMetricResponseFilter {
    /// The log-based metric filter. Logs matching this filter will be aggregated in this metric.
    pub fn new() -> LogsMetricResponseFilter {
        LogsMetricResponseFilter { query: None }
    }
}
