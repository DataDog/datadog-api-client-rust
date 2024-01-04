// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident todo's attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoAttributes {
    /// Array of todo assignees.
    #[serde(rename = "assignees")]
    pub assignees: Vec<crate::datadogV2::model::IncidentTodoAssignee>,
    /// Timestamp when the todo was completed.
    #[serde(
        rename = "completed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub completed: Option<Option<String>>,
    /// The follow-up task's content.
    #[serde(rename = "content")]
    pub content: String,
    /// Timestamp when the incident todo was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Timestamp when the todo should be completed by.
    #[serde(
        rename = "due_date",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub due_date: Option<Option<String>>,
    /// UUID of the incident this todo is connected to.
    #[serde(rename = "incident_id")]
    pub incident_id: Option<String>,
    /// Timestamp when the incident todo was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
}

impl IncidentTodoAttributes {
    pub fn new(
        assignees: Vec<crate::datadogV2::model::IncidentTodoAssignee>,
        content: String,
    ) -> IncidentTodoAttributes {
        IncidentTodoAttributes {
            assignees,
            completed: None,
            content,
            created: None,
            due_date: None,
            incident_id: None,
            modified: None,
        }
    }
}
