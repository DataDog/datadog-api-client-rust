// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Teams response metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsResponseMetaPagination {
    /// The first offset.
    #[serde(rename = "first_offset")]
    pub first_offset: Option<i64>,
    /// The last offset.
    #[serde(rename = "last_offset")]
    pub last_offset: Option<i64>,
    /// Pagination limit.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The next offset.
    #[serde(rename = "next_offset")]
    pub next_offset: Option<i64>,
    /// The offset.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// The previous offset.
    #[serde(rename = "prev_offset")]
    pub prev_offset: Option<i64>,
    /// Total results.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Offset type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl TeamsResponseMetaPagination {
    pub fn new() -> TeamsResponseMetaPagination {
        TeamsResponseMetaPagination {
            first_offset: None,
            last_offset: None,
            limit: None,
            next_offset: None,
            offset: None,
            prev_offset: None,
            total: None,
            type_: None,
        }
    }

    pub fn with_first_offset(&mut self, value: i64) -> &mut Self {
        self.first_offset = Some(value);
        self
    }

    pub fn with_last_offset(&mut self, value: i64) -> &mut Self {
        self.last_offset = Some(value);
        self
    }

    pub fn with_limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn with_next_offset(&mut self, value: i64) -> &mut Self {
        self.next_offset = Some(value);
        self
    }

    pub fn with_offset(&mut self, value: i64) -> &mut Self {
        self.offset = Some(value);
        self
    }

    pub fn with_prev_offset(&mut self, value: i64) -> &mut Self {
        self.prev_offset = Some(value);
        self
    }

    pub fn with_total(&mut self, value: i64) -> &mut Self {
        self.total = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for TeamsResponseMetaPagination {
    fn default() -> Self {
        Self::new()
    }
}
