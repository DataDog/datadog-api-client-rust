// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringSignalArchiveReason {
    NONE,
    FALSE_POSITIVE,
    TESTING_OR_MAINTENANCE,
    INVESTIGATED_CASE_OPENED,
    TRUE_POSITIVE_BENIGN,
    TRUE_POSITIVE_MALICIOUS,
    OTHER,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringSignalArchiveReason {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("none"),
            Self::FALSE_POSITIVE => String::from("false_positive"),
            Self::TESTING_OR_MAINTENANCE => String::from("testing_or_maintenance"),
            Self::INVESTIGATED_CASE_OPENED => String::from("investigated_case_opened"),
            Self::TRUE_POSITIVE_BENIGN => String::from("true_positive_benign"),
            Self::TRUE_POSITIVE_MALICIOUS => String::from("true_positive_malicious"),
            Self::OTHER => String::from("other"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringSignalArchiveReason {
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

impl<'de> Deserialize<'de> for SecurityMonitoringSignalArchiveReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "none" => Self::NONE,
            "false_positive" => Self::FALSE_POSITIVE,
            "testing_or_maintenance" => Self::TESTING_OR_MAINTENANCE,
            "investigated_case_opened" => Self::INVESTIGATED_CASE_OPENED,
            "true_positive_benign" => Self::TRUE_POSITIVE_BENIGN,
            "true_positive_malicious" => Self::TRUE_POSITIVE_MALICIOUS,
            "other" => Self::OTHER,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
