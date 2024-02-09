// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination properties.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseMetaPagination {
    /// The index of the first element in the next page of results. Equal to page size added to the current offset.
    #[serde(rename = "next_offset")]
    pub next_offset: Option<i64>,
    /// The index of the first element in the results.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Maximum size of pages to return.
    #[serde(rename = "size")]
    pub size: Option<i64>,
}

impl IncidentResponseMetaPagination {
    pub fn new() -> IncidentResponseMetaPagination {
        IncidentResponseMetaPagination {
            next_offset: None,
            offset: None,
            size: None,
        }
    }

    pub fn next_offset(&mut self, value: i64) -> &mut Self {
        self.next_offset = Some(value);
        self
    }

    pub fn offset(&mut self, value: i64) -> &mut Self {
        self.offset = Some(value);
        self
    }

    pub fn size(&mut self, value: i64) -> &mut Self {
        self.size = Some(value);
        self
    }
}

impl Default for IncidentResponseMetaPagination {
    fn default() -> Self {
        Self::new()
    }
}
