// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentUserDefinedFieldCategory {
    WHAT_HAPPENED,
    WHY_IT_HAPPENED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentUserDefinedFieldCategory {
    fn to_string(&self) -> String {
        match self {
            Self::WHAT_HAPPENED => String::from("what_happened"),
            Self::WHY_IT_HAPPENED => String::from("why_it_happened"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentUserDefinedFieldCategory {
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

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "what_happened" => Self::WHAT_HAPPENED,
            "why_it_happened" => Self::WHY_IT_HAPPENED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
