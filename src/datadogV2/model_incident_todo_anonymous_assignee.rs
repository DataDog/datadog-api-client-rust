// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoAnonymousAssignee {
    /// URL for assignee's icon.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: String,
    /// Anonymous assignee's ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Assignee's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The source of the anonymous assignee.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: IncidentTodoAnonymousAssigneeSource,
}

