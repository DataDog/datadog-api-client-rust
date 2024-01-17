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
}
