// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoAttributes {
    /// Array of todo assignees.
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Vec<IncidentTodoAssignee>,
    /// Timestamp when the todo was completed.
    #[serde(rename = "completed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub completed: Option<String>,
    /// The follow-up task's content.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: String,
    /// Timestamp when the todo should be completed by.
    #[serde(rename = "due_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub due_date: Option<String>,
    /// UUID of the incident this todo is connected to.
    #[serde(rename = "incident_id", skip_serializing_if = "Option::is_none")]
    pub incident_id: String,
}

