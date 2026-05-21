// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentSearchIncidentsIncludeType {
    INCIDENT_TYPE,
    IMPACTS,
    USERS,
    RESPONDERS,
    INTEGRATIONS,
    ATTACHMENTS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentSearchIncidentsIncludeType {
    fn to_string(&self) -> String {
        match self {
            Self::INCIDENT_TYPE => String::from("incident_type"),
            Self::IMPACTS => String::from("impacts"),
            Self::USERS => String::from("users"),
            Self::RESPONDERS => String::from("responders"),
            Self::INTEGRATIONS => String::from("integrations"),
            Self::ATTACHMENTS => String::from("attachments"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentSearchIncidentsIncludeType {
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

impl<'de> Deserialize<'de> for IncidentSearchIncidentsIncludeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "incident_type" => Self::INCIDENT_TYPE,
            "impacts" => Self::IMPACTS,
            "users" => Self::USERS,
            "responders" => Self::RESPONDERS,
            "integrations" => Self::INTEGRATIONS,
            "attachments" => Self::ATTACHMENTS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
