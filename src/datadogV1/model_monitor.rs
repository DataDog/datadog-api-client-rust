// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Monitor {
    /// Timestamp of the monitor creation.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Creator,
    /// Whether or not the monitor is deleted. (Always `null`)
    #[serde(rename = "deleted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub deleted: Option<Time>,
    /// ID of this monitor.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// A list of active downtimes that match this monitor.
    #[serde(rename = "matching_downtimes", skip_serializing_if = "Option::is_none")]
    pub matching_downtimes: Vec<MatchingDowntime>,
    /// A message to include with notifications for this monitor.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// Last timestamp when the monitor was edited.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// Whether or not the monitor is broken down on different groups.
    #[serde(rename = "multi", skip_serializing_if = "Option::is_none")]
    pub multi: bool,
    /// The monitor name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// List of options associated with your monitor.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: MonitorOptions,
    /// The different states your monitor can be in.
    #[serde(rename = "overall_state", skip_serializing_if = "Option::is_none")]
    pub overall_state: MonitorOverallStates,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub priority: Option<Int64>,
    /// The monitor query.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// A list of unique role identifiers to define which roles are allowed to edit the monitor. The unique identifiers for all roles can be pulled from the [Roles API](https://docs.datadoghq.com/api/latest/roles/#list-roles) and are located in the `data.id` field. Editing a monitor includes any updates to the monitor configuration, monitor deletion, and muting of the monitor for any amount of time. `restricted_roles` is the successor of `locked`. For more information about `locked` and `restricted_roles`, see the [monitor options docs](https://docs.datadoghq.com/monitors/guide/monitor_api_options/#permissions-options).
    #[serde(rename = "restricted_roles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub restricted_roles: datadog.NullableList[String],
    /// Wrapper object with the different monitor states.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: MonitorState,
    /// Tags associated to your monitor.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The type of the monitor. For more information about `type`, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MonitorType,
}

