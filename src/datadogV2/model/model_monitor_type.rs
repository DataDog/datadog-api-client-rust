// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes from the monitor that triggered the event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    pub fn created_at(&mut self, value: i64) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn group_status(&mut self, value: i32) -> &mut Self {
        self.group_status = Some(value);
        self
    }

    pub fn groups(&mut self, value: Vec<String>) -> &mut Self {
        self.groups = Some(value);
        self
    }

    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn modified(&mut self, value: i64) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn templated_name(&mut self, value: String) -> &mut Self {
        self.templated_name = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MonitorType {
    fn default() -> Self {
        Self::new()
    }
}
