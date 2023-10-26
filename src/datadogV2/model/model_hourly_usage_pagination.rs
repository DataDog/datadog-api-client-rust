// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsagePagination {
    /// The cursor to get the next results (if any). To make the next request, use the same parameters and add `next_record_id`.
    #[serde(
        rename = "next_record_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_record_id: Option<Option<String>>,
}

impl HourlyUsagePagination {
    /// The metadata for the current pagination.
    pub fn new() -> HourlyUsagePagination {
        HourlyUsagePagination { next_record_id: None }
    }
}
