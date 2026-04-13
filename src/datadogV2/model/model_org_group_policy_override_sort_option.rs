// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrgGroupPolicyOverrideSortOption {
    ID,
    MINUS_ID,
    ORG_UUID,
    MINUS_ORG_UUID,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OrgGroupPolicyOverrideSortOption {
    fn to_string(&self) -> String {
        match self {
            Self::ID => String::from("id"),
            Self::MINUS_ID => String::from("-id"),
            Self::ORG_UUID => String::from("org_uuid"),
            Self::MINUS_ORG_UUID => String::from("-org_uuid"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OrgGroupPolicyOverrideSortOption {
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

impl<'de> Deserialize<'de> for OrgGroupPolicyOverrideSortOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "id" => Self::ID,
            "-id" => Self::MINUS_ID,
            "org_uuid" => Self::ORG_UUID,
            "-org_uuid" => Self::MINUS_ORG_UUID,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
