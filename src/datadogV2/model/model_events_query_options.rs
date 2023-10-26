// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventsQueryOptions {
    /// The time offset to apply to the query in seconds.
    #[serde(rename = "timeOffset", skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i64>,
    /// The timezone can be specified as GMT, UTC, an offset from UTC (like UTC+1), or as a Timezone Database identifier (like America/New_York).
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl EventsQueryOptions {
    /// The global query options that are used. Either provide a timezone or a time offset but not both,
    /// otherwise the query fails.
    pub fn new() -> EventsQueryOptions {
        EventsQueryOptions {
            time_offset: None,
            timezone: None,
        }
    }
}
