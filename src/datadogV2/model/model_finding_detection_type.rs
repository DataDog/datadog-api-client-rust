// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FindingDetectionType {
    MISCONFIGURATION,
    ATTACK_PATH,
    IDENTITY_RISK,
    API_SECURITY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FindingDetectionType {
    fn to_string(&self) -> String {
        match self {
            Self::MISCONFIGURATION => String::from("misconfiguration"),
            Self::ATTACK_PATH => String::from("attack_path"),
            Self::IDENTITY_RISK => String::from("identity_risk"),
            Self::API_SECURITY => String::from("api_security"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FindingDetectionType {
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

impl<'de> Deserialize<'de> for FindingDetectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "misconfiguration" => Self::MISCONFIGURATION,
            "attack_path" => Self::ATTACK_PATH,
            "identity_risk" => Self::IDENTITY_RISK,
            "api_security" => Self::API_SECURITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
