// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The most recent actual start and end dates for a recurring downtime. For a canceled downtime,
/// this is the previously occurring downtime. For active downtimes, this is the ongoing downtime, and for scheduled
/// downtimes it is the upcoming downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleCurrentDowntimeResponse {
    /// The end of the current downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<String>>,
    /// The start of the current downtime.
    #[serde(rename = "start")]
    pub start: Option<String>,
}

impl DowntimeScheduleCurrentDowntimeResponse {
    pub fn new() -> DowntimeScheduleCurrentDowntimeResponse {
        DowntimeScheduleCurrentDowntimeResponse {
            end: None,
            start: None,
        }
    }

    pub fn end(mut self, value: Option<String>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn start(mut self, value: String) -> Self {
        self.start = Some(value);
        self
    }
}

impl Default for DowntimeScheduleCurrentDowntimeResponse {
    fn default() -> Self {
        Self::new()
    }
}
