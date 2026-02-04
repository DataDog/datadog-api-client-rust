// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringContentPackActivation {
    NEVER_ACTIVATED,
    ACTIVATED,
    DEACTIVATED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringContentPackActivation {
    fn to_string(&self) -> String {
        match self {
            Self::NEVER_ACTIVATED => String::from("never_activated"),
            Self::ACTIVATED => String::from("activated"),
            Self::DEACTIVATED => String::from("deactivated"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringContentPackActivation {
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

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackActivation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "never_activated" => Self::NEVER_ACTIVATED,
            "activated" => Self::ACTIVATED,
            "deactivated" => Self::DEACTIVATED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
