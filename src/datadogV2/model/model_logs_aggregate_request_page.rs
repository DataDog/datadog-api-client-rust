// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paging settings
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateRequestPage {
    /// The returned paging point to use to get the next results. Note: at most 1000 results can be paged.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
}

impl LogsAggregateRequestPage {
    pub fn new() -> LogsAggregateRequestPage {
        LogsAggregateRequestPage { cursor: None }
    }
}
impl Default for LogsAggregateRequestPage {
    fn default() -> Self {
        Self::new()
    }
}
