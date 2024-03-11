// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleDetectionMethod {
    THRESHOLD,
    NEW_VALUE,
    ANOMALY_DETECTION,
    IMPOSSIBLE_TRAVEL,
    HARDCODED,
    THIRD_PARTY,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SecurityMonitoringRuleDetectionMethod {
    fn to_string(&self) -> String {
        match self {
            Self::THRESHOLD => String::from("threshold"),
            Self::NEW_VALUE => String::from("new_value"),
            Self::ANOMALY_DETECTION => String::from("anomaly_detection"),
            Self::IMPOSSIBLE_TRAVEL => String::from("impossible_travel"),
            Self::HARDCODED => String::from("hardcoded"),
            Self::THIRD_PARTY => String::from("third_party"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleDetectionMethod {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleDetectionMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "threshold" => Self::THRESHOLD,
            "new_value" => Self::NEW_VALUE,
            "anomaly_detection" => Self::ANOMALY_DETECTION,
            "impossible_travel" => Self::IMPOSSIBLE_TRAVEL,
            "hardcoded" => Self::HARDCODED,
            "third_party" => Self::THIRD_PARTY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
