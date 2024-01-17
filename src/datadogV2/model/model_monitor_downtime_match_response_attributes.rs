// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Downtime match details.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponseAttributes {
    /// The end of the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<String>>,
    /// An array of groups associated with the downtime.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The start of the downtime.
    #[serde(rename = "start")]
    pub start: Option<String>,
}

impl MonitorDowntimeMatchResponseAttributes {
    pub fn new() -> MonitorDowntimeMatchResponseAttributes {
        MonitorDowntimeMatchResponseAttributes {
            end: None,
            groups: None,
            scope: None,
            start: None,
        }
    }
}
impl Default for MonitorDowntimeMatchResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
