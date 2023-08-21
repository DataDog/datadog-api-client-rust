// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsQueryOptions {
    /// The time offset to apply to the query in seconds.
    #[serde(rename = "timeOffset", skip_serializing_if = "Option::is_none")]
    pub time_offset: i64,
    /// The timezone can be specified as GMT, UTC, an offset from UTC (like UTC+1), or as a Timezone Database identifier (like America/New_York).
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
}

