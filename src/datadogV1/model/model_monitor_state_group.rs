// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Monitor state for a single group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorStateGroup {
    /// Latest timestamp the monitor was in NO_DATA state.
    #[serde(rename = "last_nodata_ts")]
    pub last_nodata_ts: Option<i64>,
    /// Latest timestamp of the notification sent for this monitor group.
    #[serde(rename = "last_notified_ts")]
    pub last_notified_ts: Option<i64>,
    /// Latest timestamp the monitor group was resolved.
    #[serde(rename = "last_resolved_ts")]
    pub last_resolved_ts: Option<i64>,
    /// Latest timestamp the monitor group triggered.
    #[serde(rename = "last_triggered_ts")]
    pub last_triggered_ts: Option<i64>,
    /// The name of the monitor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The different states your monitor can be in.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::MonitorOverallStates>,
}

impl MonitorStateGroup {
    pub fn new() -> MonitorStateGroup {
        MonitorStateGroup {
            last_nodata_ts: None,
            last_notified_ts: None,
            last_resolved_ts: None,
            last_triggered_ts: None,
            name: None,
            status: None,
        }
    }

    pub fn last_nodata_ts(mut self, value: i64) -> Self {
        self.last_nodata_ts = Some(value);
        self
    }

    pub fn last_notified_ts(mut self, value: i64) -> Self {
        self.last_notified_ts = Some(value);
        self
    }

    pub fn last_resolved_ts(mut self, value: i64) -> Self {
        self.last_resolved_ts = Some(value);
        self
    }

    pub fn last_triggered_ts(mut self, value: i64) -> Self {
        self.last_triggered_ts = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::MonitorOverallStates) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for MonitorStateGroup {
    fn default() -> Self {
        Self::new()
    }
}
