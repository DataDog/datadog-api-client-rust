// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaseStatusGroup {
    SG_OPEN,
    SG_IN_PROGRESS,
    SG_CLOSED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CaseStatusGroup {
    fn to_string(&self) -> String {
        match self {
            Self::SG_OPEN => String::from("SG_OPEN"),
            Self::SG_IN_PROGRESS => String::from("SG_IN_PROGRESS"),
            Self::SG_CLOSED => String::from("SG_CLOSED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CaseStatusGroup {
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

impl<'de> Deserialize<'de> for CaseStatusGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "SG_OPEN" => Self::SG_OPEN,
            "SG_IN_PROGRESS" => Self::SG_IN_PROGRESS,
            "SG_CLOSED" => Self::SG_CLOSED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
