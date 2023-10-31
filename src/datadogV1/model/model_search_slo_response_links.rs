// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination links.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseLinks {
    /// Link to last page.
    #[serde(rename = "first")]
    pub first: Option<String>,
    /// Link to first page.
    #[serde(rename = "last", default, with = "::serde_with::rust::double_option")]
    pub last: Option<Option<String>>,
    /// Link to the next page.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Link to previous page.
    #[serde(rename = "prev", default, with = "::serde_with::rust::double_option")]
    pub prev: Option<Option<String>>,
    /// Link to current page.
    #[serde(rename = "self")]
    pub self_: Option<String>,
}

impl SearchSLOResponseLinks {
    pub fn new() -> SearchSLOResponseLinks {
        SearchSLOResponseLinks {
            first: None,
            last: None,
            next: None,
            prev: None,
            self_: None,
        }
    }
}
