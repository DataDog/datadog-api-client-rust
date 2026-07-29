// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentRuleTriggerType {
    INCIDENT_SAVED_TRIGGER,
    INCIDENT_CREATED_TRIGGER,
    INCIDENT_MODIFIED_TRIGGER,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentRuleTriggerType {
    fn to_string(&self) -> String {
        match self {
            Self::INCIDENT_SAVED_TRIGGER => String::from("incident_saved_trigger"),
            Self::INCIDENT_CREATED_TRIGGER => String::from("incident_created_trigger"),
            Self::INCIDENT_MODIFIED_TRIGGER => String::from("incident_modified_trigger"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentRuleTriggerType {
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

impl<'de> Deserialize<'de> for IncidentRuleTriggerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "incident_saved_trigger" => Self::INCIDENT_SAVED_TRIGGER,
            "incident_created_trigger" => Self::INCIDENT_CREATED_TRIGGER,
            "incident_modified_trigger" => Self::INCIDENT_MODIFIED_TRIGGER,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
