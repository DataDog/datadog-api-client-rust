// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata for the current pagination.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The cursor to get the next results (if any). To make the next request, use the same parameters and add `next_record_id`.
    #[serde(
        rename = "next_record_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_record_id: Option<Option<String>>,
    /// Total number of records.
    #[serde(
        rename = "total_number_of_records",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_number_of_records: Option<Option<i64>>,
}

impl UsageTopAvgMetricsPagination {
    pub fn new() -> UsageTopAvgMetricsPagination {
        UsageTopAvgMetricsPagination {
            limit: None,
            next_record_id: None,
            total_number_of_records: None,
        }
    }

    pub fn with_limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn with_next_record_id(&mut self, value: Option<String>) -> &mut Self {
        self.next_record_id = Some(value);
        self
    }

    pub fn with_total_number_of_records(&mut self, value: Option<i64>) -> &mut Self {
        self.total_number_of_records = Some(value);
        self
    }
}
impl Default for UsageTopAvgMetricsPagination {
    fn default() -> Self {
        Self::new()
    }
}
