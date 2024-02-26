// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Links attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListResponseLinks {
    /// Link for the next set of results. Note that the request can also be made using the
    /// POST endpoint.
    #[serde(rename = "next")]
    pub next: Option<String>,
}

impl LogsListResponseLinks {
    pub fn new() -> LogsListResponseLinks {
        LogsListResponseLinks { next: None }
    }

    pub fn next(&mut self, value: String) -> &mut Self {
        self.next = Some(value);
        self
    }
}

impl Default for LogsListResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}
