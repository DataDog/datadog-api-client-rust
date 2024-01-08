// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Global query options that are used during the query.
/// Note: Only supply timezone or time offset, not both. Otherwise, the query fails.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMQueryOptions {
    /// The time offset (in seconds) to apply to the query.
    #[serde(rename = "time_offset")]
    pub time_offset: Option<i64>,
    /// The timezone can be specified as GMT, UTC, an offset from UTC (like UTC+1), or as a Timezone Database identifier (like America/New_York).
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

impl RUMQueryOptions {
    pub fn new() -> RUMQueryOptions {
        RUMQueryOptions {
            time_offset: None,
            timezone: None,
        }
    }
}
impl Default for RUMQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
