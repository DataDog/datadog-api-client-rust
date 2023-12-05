// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination and findings count information.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListFindingsPage {
    /// The cursor used to paginate requests.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// The total count of findings after the filter has been applied.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
}

impl ListFindingsPage {
    pub fn new() -> ListFindingsPage {
        ListFindingsPage {
            cursor: None,
            total_filtered_count: None,
        }
    }
}