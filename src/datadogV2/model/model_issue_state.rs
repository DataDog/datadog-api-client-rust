// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueState {
    OPEN,
    ACKNOWLEDGED,
    RESOLVED,
    IGNORED,
    EXCLUDED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssueState {
    fn to_string(&self) -> String {
        match self {
            Self::OPEN => String::from("OPEN"),
            Self::ACKNOWLEDGED => String::from("ACKNOWLEDGED"),
            Self::RESOLVED => String::from("RESOLVED"),
            Self::IGNORED => String::from("IGNORED"),
            Self::EXCLUDED => String::from("EXCLUDED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IssueState {
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

impl<'de> Deserialize<'de> for IssueState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "OPEN" => Self::OPEN,
            "ACKNOWLEDGED" => Self::ACKNOWLEDGED,
            "RESOLVED" => Self::RESOLVED,
            "IGNORED" => Self::IGNORED,
            "EXCLUDED" => Self::EXCLUDED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
