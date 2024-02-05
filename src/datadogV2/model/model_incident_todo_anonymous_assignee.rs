// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Anonymous assignee entity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoAnonymousAssignee {
    /// URL for assignee's icon.
    #[serde(rename = "icon")]
    pub icon: String,
    /// Anonymous assignee's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Assignee's name.
    #[serde(rename = "name")]
    pub name: String,
    /// The source of the anonymous assignee.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::IncidentTodoAnonymousAssigneeSource,
}

impl IncidentTodoAnonymousAssignee {
    pub fn new(
        icon: String,
        id: String,
        name: String,
        source: crate::datadogV2::model::IncidentTodoAnonymousAssigneeSource,
    ) -> IncidentTodoAnonymousAssignee {
        IncidentTodoAnonymousAssignee {
            icon,
            id,
            name,
            source,
        }
    }
}
