// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentImportRelatedObject {
    LAST_MODIFIED_BY_USER,
    CREATED_BY_USER,
    COMMANDER_USER,
    DECLARED_BY_USER,
    INCIDENT_TYPE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentImportRelatedObject {
    fn to_string(&self) -> String {
        match self {
            Self::LAST_MODIFIED_BY_USER => String::from("last_modified_by_user"),
            Self::CREATED_BY_USER => String::from("created_by_user"),
            Self::COMMANDER_USER => String::from("commander_user"),
            Self::DECLARED_BY_USER => String::from("declared_by_user"),
            Self::INCIDENT_TYPE => String::from("incident_type"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentImportRelatedObject {
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

impl<'de> Deserialize<'de> for IncidentImportRelatedObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "last_modified_by_user" => Self::LAST_MODIFIED_BY_USER,
            "created_by_user" => Self::CREATED_BY_USER,
            "commander_user" => Self::COMMANDER_USER,
            "declared_by_user" => Self::DECLARED_BY_USER,
            "incident_type" => Self::INCIDENT_TYPE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
