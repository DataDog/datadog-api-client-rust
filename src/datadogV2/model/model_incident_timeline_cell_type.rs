// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentTimelineCellType {
    MARKDOWN,
    INCIDENT_STATUS_CHANGE,
    TIMESTAMP_CHANGE,
    MEETING_SUMMARY,
    MEETING_CHAT,
    ROLE_ASSIGNMENT_CHANGE,
    POSTMORTEM_CHANGE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentTimelineCellType {
    fn to_string(&self) -> String {
        match self {
            Self::MARKDOWN => String::from("markdown"),
            Self::INCIDENT_STATUS_CHANGE => String::from("incident_status_change"),
            Self::TIMESTAMP_CHANGE => String::from("timestamp_change"),
            Self::MEETING_SUMMARY => String::from("meeting_summary"),
            Self::MEETING_CHAT => String::from("meeting_chat"),
            Self::ROLE_ASSIGNMENT_CHANGE => String::from("role_assignment_change"),
            Self::POSTMORTEM_CHANGE => String::from("postmortem_change"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentTimelineCellType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for IncidentTimelineCellType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "markdown" => Self::MARKDOWN,
            "incident_status_change" => Self::INCIDENT_STATUS_CHANGE,
            "timestamp_change" => Self::TIMESTAMP_CHANGE,
            "meeting_summary" => Self::MEETING_SUMMARY,
            "meeting_chat" => Self::MEETING_CHAT,
            "role_assignment_change" => Self::ROLE_ASSIGNMENT_CHANGE,
            "postmortem_change" => Self::POSTMORTEM_CHANGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
