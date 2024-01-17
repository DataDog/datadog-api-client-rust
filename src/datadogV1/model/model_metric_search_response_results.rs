// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Search result.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSearchResponseResults {
    /// List of metrics that match the search query.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
}

impl MetricSearchResponseResults {
    pub fn new() -> MetricSearchResponseResults {
        MetricSearchResponseResults { metrics: None }
    }
}
