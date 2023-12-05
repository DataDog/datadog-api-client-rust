// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination settings.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventsRequestPage {
    /// The returned paging point to use to get the next results.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// The maximum number of logs in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
}

impl EventsRequestPage {
    pub fn new() -> EventsRequestPage {
        EventsRequestPage {
            cursor: None,
            limit: None,
        }
    }
}