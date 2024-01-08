// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The search and filter query settings.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansQueryFilter {
    /// The minimum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// The search query - following the span search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The maximum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl SpansQueryFilter {
    pub fn new() -> SpansQueryFilter {
        SpansQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }
}
impl Default for SpansQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}
