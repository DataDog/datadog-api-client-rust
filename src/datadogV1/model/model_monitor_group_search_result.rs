// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single monitor group search result.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorGroupSearchResult {
    /// The name of the group.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// The list of tags of the monitor group.
    #[serde(rename = "group_tags")]
    pub group_tags: Option<Vec<String>>,
    /// Latest timestamp the monitor group was in NO_DATA state.
    #[serde(rename = "last_nodata_ts")]
    pub last_nodata_ts: Option<i64>,
    /// Latest timestamp the monitor group triggered.
    #[serde(
        rename = "last_triggered_ts",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_triggered_ts: Option<Option<i64>>,
    /// The ID of the monitor.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// The name of the monitor.
    #[serde(rename = "monitor_name")]
    pub monitor_name: Option<String>,
    /// The different states your monitor can be in.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::MonitorOverallStates>,
}

impl MonitorGroupSearchResult {
    pub fn new() -> MonitorGroupSearchResult {
        MonitorGroupSearchResult {
            group: None,
            group_tags: None,
            last_nodata_ts: None,
            last_triggered_ts: None,
            monitor_id: None,
            monitor_name: None,
            status: None,
        }
    }
}
impl Default for MonitorGroupSearchResult {
    fn default() -> Self {
        Self::new()
    }
}
