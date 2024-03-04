// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paging attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummariesMetaPage {
    /// The cursor used to get the next results, if any. To make the next request, use the same
    /// parameters with the addition of the `page[cursor]`.
    #[serde(rename = "after")]
    pub after: Option<String>,
    /// Number of results returned.
    #[serde(rename = "size")]
    pub size: Option<i32>,
}

impl ProcessSummariesMetaPage {
    pub fn new() -> ProcessSummariesMetaPage {
        ProcessSummariesMetaPage {
            after: None,
            size: None,
        }
    }

    pub fn after(mut self, value: String) -> Self {
        self.after = Some(value);
        self
    }

    pub fn size(mut self, value: i32) -> Self {
        self.size = Some(value);
        self
    }
}

impl Default for ProcessSummariesMetaPage {
    fn default() -> Self {
        Self::new()
    }
}
