// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination links.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImagesResponseLinks {
    /// Link to the first page.
    #[serde(rename = "first")]
    pub first: Option<String>,
    /// Link to the last page.
    #[serde(rename = "last", default, with = "::serde_with::rust::double_option")]
    pub last: Option<Option<String>>,
    /// Link to the next page.
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option")]
    pub next: Option<Option<String>>,
    /// Link to previous page.
    #[serde(rename = "prev", default, with = "::serde_with::rust::double_option")]
    pub prev: Option<Option<String>>,
    /// Link to current page.
    #[serde(rename = "self")]
    pub self_: Option<String>,
}

impl ContainerImagesResponseLinks {
    pub fn new() -> ContainerImagesResponseLinks {
        ContainerImagesResponseLinks {
            first: None,
            last: None,
            next: None,
            prev: None,
            self_: None,
        }
    }

    pub fn first(&mut self, value: String) -> &mut Self {
        self.first = Some(value);
        self
    }

    pub fn last(&mut self, value: Option<String>) -> &mut Self {
        self.last = Some(value);
        self
    }

    pub fn next(&mut self, value: Option<String>) -> &mut Self {
        self.next = Some(value);
        self
    }

    pub fn prev(&mut self, value: Option<String>) -> &mut Self {
        self.prev = Some(value);
        self
    }

    pub fn self_(&mut self, value: String) -> &mut Self {
        self.self_ = Some(value);
        self
    }
}

impl Default for ContainerImagesResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}
