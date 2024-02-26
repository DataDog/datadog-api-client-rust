// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The spans filter used to index spans.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansFilter {
    /// The search query - following the [span search syntax](<https://docs.datadoghq.com/tracing/trace_explorer/query_syntax/>).
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SpansFilter {
    pub fn new() -> SpansFilter {
        SpansFilter { query: None }
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }
}

impl Default for SpansFilter {
    fn default() -> Self {
        Self::new()
    }
}
