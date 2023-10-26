// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummariesMetaPage {
    /// The cursor used to get the next results, if any. To make the next request, use the same
    /// parameters with the addition of the `page[cursor]`.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results returned.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ProcessSummariesMetaPage {
    /// Paging attributes.
    pub fn new() -> ProcessSummariesMetaPage {
        ProcessSummariesMetaPage {
            after: None,
            size: None,
        }
    }
}
