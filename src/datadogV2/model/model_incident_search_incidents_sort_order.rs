// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentSearchIncidentsSortOrder {
    CREATED_ASCENDING,
    CREATED_DESCENDING,
    MODIFIED_ASCENDING,
    MODIFIED_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentSearchIncidentsSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED_ASCENDING => String::from("created"),
            Self::CREATED_DESCENDING => String::from("-created"),
            Self::MODIFIED_ASCENDING => String::from("modified"),
            Self::MODIFIED_DESCENDING => String::from("-modified"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentSearchIncidentsSortOrder {
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

impl<'de> Deserialize<'de> for IncidentSearchIncidentsSortOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created" => Self::CREATED_ASCENDING,
            "-created" => Self::CREATED_DESCENDING,
            "modified" => Self::MODIFIED_ASCENDING,
            "-modified" => Self::MODIFIED_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
