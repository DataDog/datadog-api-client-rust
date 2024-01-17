// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Filter for logs.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsFilter {
    /// The filter query.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl LogsFilter {
    pub fn new() -> LogsFilter {
        LogsFilter { query: None }
    }
}
impl Default for LogsFilter {
    fn default() -> Self {
        Self::new()
    }
}
