// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing page total count for specified ID.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsPage {
    /// Total page count.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
}

impl UsageSpecifiedCustomReportsPage {
    pub fn new() -> UsageSpecifiedCustomReportsPage {
        UsageSpecifiedCustomReportsPage { total_count: None }
    }
}
impl Default for UsageSpecifiedCustomReportsPage {
    fn default() -> Self {
        Self::new()
    }
}
