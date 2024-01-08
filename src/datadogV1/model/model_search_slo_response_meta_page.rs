// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination metadata returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseMetaPage {
    /// The first number.
    #[serde(rename = "first_number")]
    pub first_number: Option<i64>,
    /// The last number.
    #[serde(rename = "last_number")]
    pub last_number: Option<i64>,
    /// The next number.
    #[serde(rename = "next_number")]
    pub next_number: Option<i64>,
    /// The page number.
    #[serde(rename = "number")]
    pub number: Option<i64>,
    /// The previous page number.
    #[serde(rename = "prev_number")]
    pub prev_number: Option<i64>,
    /// The size of the response.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// The total number of SLOs in the response.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Type of pagination.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl SearchSLOResponseMetaPage {
    pub fn new() -> SearchSLOResponseMetaPage {
        SearchSLOResponseMetaPage {
            first_number: None,
            last_number: None,
            next_number: None,
            number: None,
            prev_number: None,
            size: None,
            total: None,
            type_: None,
        }
    }
}
impl Default for SearchSLOResponseMetaPage {
    fn default() -> Self {
        Self::new()
    }
}
