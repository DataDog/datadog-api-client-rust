// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MonitorType {
    /// The POSIX timestamp of the monitor's creation in nanoseconds.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// Monitor group status used when there is no `result_groups`.
    #[serde(rename = "group_status")]
    pub group_status: Option<i32>,
    /// Groups to which the monitor belongs.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,
    /// The monitor ID.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The monitor message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The monitor's last-modified timestamp.
    #[serde(rename = "modified")]
    pub modified: Option<i64>,
    /// The monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The query that triggers the alert.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// A list of tags attached to the monitor.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The templated name of the monitor before resolving any template variables.
    #[serde(rename = "templated_name")]
    pub templated_name: Option<String>,
    /// The monitor type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl MonitorType {
    /// Attributes from the monitor that triggered the event.
    pub fn new() -> MonitorType {
        MonitorType {
            created_at: None,
            group_status: None,
            groups: None,
            id: None,
            message: None,
            modified: None,
            name: None,
            query: None,
            tags: None,
            templated_name: None,
            type_: None,
        }
    }
}
