// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TriggerSource {
    SECURITY_FINDINGS,
    SECURITY_SIGNALS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TriggerSource {
    fn to_string(&self) -> String {
        match self {
            Self::SECURITY_FINDINGS => String::from("security_findings"),
            Self::SECURITY_SIGNALS => String::from("security_signals"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TriggerSource {
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

impl<'de> Deserialize<'de> for TriggerSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "security_findings" => Self::SECURITY_FINDINGS,
            "security_signals" => Self::SECURITY_SIGNALS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
