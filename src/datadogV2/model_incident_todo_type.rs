// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentTodoType {
    #[serde(rename = "incident_todos")]
	INCIDENT_TODOS,
}

impl ToString for IncidentTodoType {
    fn to_string(&self) -> String {
        match self {
            Self::INCIDENT_TODOS => String::from("incident_todos"),
        }
    }
}

impl Default for IncidentTodoType {
    fn default() -> IncidentTodoType {
        Self::INCIDENT_TODOS
    }
}
