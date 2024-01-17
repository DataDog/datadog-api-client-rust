// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A one-time downtime definition.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleOneTimeCreateUpdateRequest {
    /// ISO-8601 Datetime to end the downtime. Must include a UTC offset of zero. If not provided, the
    /// downtime continues forever.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<String>>,
    /// ISO-8601 Datetime to start the downtime. Must include a UTC offset of zero. If not provided, the
    /// downtime starts the moment it is created.
    #[serde(rename = "start", default, with = "::serde_with::rust::double_option")]
    pub start: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}

impl DowntimeScheduleOneTimeCreateUpdateRequest {
    pub fn new() -> DowntimeScheduleOneTimeCreateUpdateRequest {
        DowntimeScheduleOneTimeCreateUpdateRequest {
            end: None,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
        }
    }
}
impl Default for DowntimeScheduleOneTimeCreateUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
