// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationSecurityWafCustomRuleTagsCategory {
    ATTACK_ATTEMPT,
    BUSINESS_LOGIC,
    SECURITY_RESPONSES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ApplicationSecurityWafCustomRuleTagsCategory {
    fn to_string(&self) -> String {
        match self {
            Self::ATTACK_ATTEMPT => String::from("attack_attempt"),
            Self::BUSINESS_LOGIC => String::from("business_logic"),
            Self::SECURITY_RESPONSES => String::from("security_responses"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ApplicationSecurityWafCustomRuleTagsCategory {
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

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleTagsCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "attack_attempt" => Self::ATTACK_ATTEMPT,
            "business_logic" => Self::BUSINESS_LOGIC,
            "security_responses" => Self::SECURITY_RESPONSES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
