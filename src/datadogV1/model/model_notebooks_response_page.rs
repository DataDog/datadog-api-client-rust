// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination metadata returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponsePage {
    /// The total number of notebooks that would be returned if the request was not filtered by `start` and `count` parameters.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    /// The total number of notebooks returned.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
}

impl NotebooksResponsePage {
    pub fn new() -> NotebooksResponsePage {
        NotebooksResponsePage {
            total_count: None,
            total_filtered_count: None,
        }
    }
}
impl Default for NotebooksResponsePage {
    fn default() -> Self {
        Self::new()
    }
}
