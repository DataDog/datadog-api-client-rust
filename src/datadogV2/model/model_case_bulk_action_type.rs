// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaseBulkActionType {
    PRIORITY,
    STATUS,
    ASSIGN,
    UNASSIGN,
    ARCHIVE,
    UNARCHIVE,
    JIRA,
    SERVICENOW,
    LINEAR,
    UPDATE_PROJECT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CaseBulkActionType {
    fn to_string(&self) -> String {
        match self {
            Self::PRIORITY => String::from("priority"),
            Self::STATUS => String::from("status"),
            Self::ASSIGN => String::from("assign"),
            Self::UNASSIGN => String::from("unassign"),
            Self::ARCHIVE => String::from("archive"),
            Self::UNARCHIVE => String::from("unarchive"),
            Self::JIRA => String::from("jira"),
            Self::SERVICENOW => String::from("servicenow"),
            Self::LINEAR => String::from("linear"),
            Self::UPDATE_PROJECT => String::from("update_project"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CaseBulkActionType {
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

impl<'de> Deserialize<'de> for CaseBulkActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "priority" => Self::PRIORITY,
            "status" => Self::STATUS,
            "assign" => Self::ASSIGN,
            "unassign" => Self::UNASSIGN,
            "archive" => Self::ARCHIVE,
            "unarchive" => Self::UNARCHIVE,
            "jira" => Self::JIRA,
            "servicenow" => Self::SERVICENOW,
            "linear" => Self::LINEAR,
            "update_project" => Self::UPDATE_PROJECT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
