// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnCallPageTargetRelationshipDataType {
    TEAMS,
    USERS,
    CROSS_ORG_HANDLES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OnCallPageTargetRelationshipDataType {
    fn to_string(&self) -> String {
        match self {
            Self::TEAMS => String::from("teams"),
            Self::USERS => String::from("users"),
            Self::CROSS_ORG_HANDLES => String::from("cross_org_handles"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OnCallPageTargetRelationshipDataType {
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

impl<'de> Deserialize<'de> for OnCallPageTargetRelationshipDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "teams" => Self::TEAMS,
            "users" => Self::USERS,
            "cross_org_handles" => Self::CROSS_ORG_HANDLES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
