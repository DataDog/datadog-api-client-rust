// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata about the response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResponseMetadata {
    /// The page to start paginating from.
    #[serde(rename = "page")]
    pub page: Option<i64>,
    /// The number of pages.
    #[serde(rename = "page_count")]
    pub page_count: Option<i64>,
    /// The number of monitors to return per page.
    #[serde(rename = "per_page")]
    pub per_page: Option<i64>,
    /// The total number of monitors.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
}

impl MonitorSearchResponseMetadata {
    pub fn new() -> MonitorSearchResponseMetadata {
        MonitorSearchResponseMetadata {
            page: None,
            page_count: None,
            per_page: None,
            total_count: None,
        }
    }
}
impl Default for MonitorSearchResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}
