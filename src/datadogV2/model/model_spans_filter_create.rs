// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The spans filter. Spans matching this filter will be indexed and stored.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansFilterCreate {
    /// The search query - following the [span search syntax](https://docs.datadoghq.com/tracing/trace_explorer/query_syntax/).
    #[serde(rename = "query")]
    pub query: String,
}

impl SpansFilterCreate {
    pub fn new(query: String) -> SpansFilterCreate {
        SpansFilterCreate { query }
    }
}