// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleCaseActionType {
    BLOCK_IP,
    BLOCK_USER,
    USER_BEHAVIOR,
    FLAG_IP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringRuleCaseActionType {
    fn to_string(&self) -> String {
        match self {
            Self::BLOCK_IP => String::from("block_ip"),
            Self::BLOCK_USER => String::from("block_user"),
            Self::USER_BEHAVIOR => String::from("user_behavior"),
            Self::FLAG_IP => String::from("flag_ip"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleCaseActionType {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleCaseActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "block_ip" => Self::BLOCK_IP,
            "block_user" => Self::BLOCK_USER,
            "user_behavior" => Self::USER_BEHAVIOR,
            "flag_ip" => Self::FLAG_IP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
