// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Teams response links.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsResponseLinks {
    /// First link.
    #[serde(rename = "first")]
    pub first: Option<String>,
    /// Last link.
    #[serde(rename = "last", default, with = "::serde_with::rust::double_option")]
    pub last: Option<Option<String>>,
    /// Next link.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Previous link.
    #[serde(rename = "prev", default, with = "::serde_with::rust::double_option")]
    pub prev: Option<Option<String>>,
    /// Current link.
    #[serde(rename = "self")]
    pub self_: Option<String>,
}

impl TeamsResponseLinks {
    pub fn new() -> TeamsResponseLinks {
        TeamsResponseLinks {
            first: None,
            last: None,
            next: None,
            prev: None,
            self_: None,
        }
    }

    pub fn with_first(&mut self, value: String) -> &mut Self {
        self.first = Some(value);
        self
    }

    pub fn with_last(&mut self, value: Option<String>) -> &mut Self {
        self.last = Some(value);
        self
    }

    pub fn with_next(&mut self, value: String) -> &mut Self {
        self.next = Some(value);
        self
    }

    pub fn with_prev(&mut self, value: Option<String>) -> &mut Self {
        self.prev = Some(value);
        self
    }

    pub fn with_self_(&mut self, value: String) -> &mut Self {
        self.self_ = Some(value);
        self
    }
}
impl Default for TeamsResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}
