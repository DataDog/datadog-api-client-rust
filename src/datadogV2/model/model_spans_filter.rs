// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansFilter {
    /// The search query - following the [span search syntax](https://docs.datadoghq.com/tracing/trace_explorer/query_syntax/).
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl SpansFilter {
    /// The spans filter used to index spans.
    pub fn new() -> SpansFilter {
        SpansFilter { query: None }
    }
}
