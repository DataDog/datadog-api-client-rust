// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorType {
    /// The POSIX timestamp of the monitor's creation in nanoseconds.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: i64,
    /// Monitor group status used when there is no `result_groups`.
    #[serde(rename = "group_status", skip_serializing_if = "Option::is_none")]
    pub group_status: i32,
    /// Groups to which the monitor belongs.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Vec<String>,
    /// The monitor ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// The monitor message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The monitor's last-modified timestamp.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: i64,
    /// The monitor name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The query that triggers the alert.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// A list of tags attached to the monitor.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The templated name of the monitor before resolving any template variables.
    #[serde(rename = "templated_name", skip_serializing_if = "Option::is_none")]
    pub templated_name: String,
    /// The monitor type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

