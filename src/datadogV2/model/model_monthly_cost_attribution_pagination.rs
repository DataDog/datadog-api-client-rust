// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata for the current pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyCostAttributionPagination {
    /// The cursor to use to get the next results, if any. To make the next request, use the same parameters with the addition of the `next_record_id`.
    #[serde(
        rename = "next_record_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_record_id: Option<Option<String>>,
}

impl MonthlyCostAttributionPagination {
    pub fn new() -> MonthlyCostAttributionPagination {
        MonthlyCostAttributionPagination {
            next_record_id: None,
        }
    }

    pub fn next_record_id(&mut self, value: Option<String>) -> &mut Self {
        self.next_record_id = Some(value);
        self
    }
}

impl Default for MonthlyCostAttributionPagination {
    fn default() -> Self {
        Self::new()
    }
}
