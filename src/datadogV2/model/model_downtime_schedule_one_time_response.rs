// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A one-time downtime definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleOneTimeResponse {
    /// ISO-8601 Datetime to end the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<String>>,
    /// ISO-8601 Datetime to start the downtime.
    #[serde(rename = "start")]
    pub start: String,
}

impl DowntimeScheduleOneTimeResponse {
    pub fn new(start: String) -> DowntimeScheduleOneTimeResponse {
        DowntimeScheduleOneTimeResponse { end: None, start }
    }

    pub fn end(&mut self, value: Option<String>) -> &mut Self {
        self.end = Some(value);
        self
    }
}
