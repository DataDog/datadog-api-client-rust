// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Links attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackResponseLinks {
    /// Link to last page.
    #[serde(rename = "first")]
    pub first: Option<String>,
    /// Link to first page.
    #[serde(rename = "last", default, with = "::serde_with::rust::double_option")]
    pub last: Option<Option<String>>,
    /// Link for the next set of results.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Link for the previous set of results.
    #[serde(rename = "prev", default, with = "::serde_with::rust::double_option")]
    pub prev: Option<Option<String>>,
    /// Link to current page.
    #[serde(rename = "self")]
    pub self_: Option<String>,
}

impl PowerpackResponseLinks {
    pub fn new() -> PowerpackResponseLinks {
        PowerpackResponseLinks {
            first: None,
            last: None,
            next: None,
            prev: None,
            self_: None,
        }
    }

    pub fn first(mut self, value: String) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: Option<String>) -> Self {
        self.last = Some(value);
        self
    }

    pub fn next(mut self, value: String) -> Self {
        self.next = Some(value);
        self
    }

    pub fn prev(mut self, value: Option<String>) -> Self {
        self.prev = Some(value);
        self
    }

    pub fn self_(mut self, value: String) -> Self {
        self.self_ = Some(value);
        self
    }
}

impl Default for PowerpackResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}
