// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata for the current pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Records to be skipped before beginning to return.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Direction to sort by.
    #[serde(rename = "sort_direction")]
    pub sort_direction: Option<String>,
    /// Field to sort by.
    #[serde(rename = "sort_name")]
    pub sort_name: Option<String>,
    /// Total number of records.
    #[serde(rename = "total_number_of_records")]
    pub total_number_of_records: Option<i64>,
}

impl UsageAttributionPagination {
    pub fn new() -> UsageAttributionPagination {
        UsageAttributionPagination {
            limit: None,
            offset: None,
            sort_direction: None,
            sort_name: None,
            total_number_of_records: None,
        }
    }

    pub fn limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(&mut self, value: i64) -> &mut Self {
        self.offset = Some(value);
        self
    }

    pub fn sort_direction(&mut self, value: String) -> &mut Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn sort_name(&mut self, value: String) -> &mut Self {
        self.sort_name = Some(value);
        self
    }

    pub fn total_number_of_records(&mut self, value: i64) -> &mut Self {
        self.total_number_of_records = Some(value);
        self
    }
}

impl Default for UsageAttributionPagination {
    fn default() -> Self {
        Self::new()
    }
}
