// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsBanditPolicyReason {
    ACTIVATION,
    REFRESH,
    ARM_CHANGE,
    SCHEDULED_END,
    STOP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsBanditPolicyReason {
    fn to_string(&self) -> String {
        match self {
            Self::ACTIVATION => String::from("ACTIVATION"),
            Self::REFRESH => String::from("REFRESH"),
            Self::ARM_CHANGE => String::from("ARM_CHANGE"),
            Self::SCHEDULED_END => String::from("SCHEDULED_END"),
            Self::STOP => String::from("STOP"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsBanditPolicyReason {
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

impl<'de> Deserialize<'de> for ExperimentsBanditPolicyReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ACTIVATION" => Self::ACTIVATION,
            "REFRESH" => Self::REFRESH,
            "ARM_CHANGE" => Self::ARM_CHANGE,
            "SCHEDULED_END" => Self::SCHEDULED_END,
            "STOP" => Self::STOP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
