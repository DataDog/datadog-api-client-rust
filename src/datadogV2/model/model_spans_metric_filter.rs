// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricFilter {
    /// The search query - following the span search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SpansMetricFilter {
    /// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
    pub fn new() -> SpansMetricFilter {
        SpansMetricFilter { query: None }
    }
}
