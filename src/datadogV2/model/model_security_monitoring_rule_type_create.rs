// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleTypeCreate {
    APPLICATION_SECURITY,
    LOG_DETECTION,
    WORKLOAD_SECURITY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringRuleTypeCreate {
    fn to_string(&self) -> String {
        match self {
            Self::APPLICATION_SECURITY => String::from("application_security"),
            Self::LOG_DETECTION => String::from("log_detection"),
            Self::WORKLOAD_SECURITY => String::from("workload_security"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleTypeCreate {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleTypeCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "application_security" => Self::APPLICATION_SECURITY,
            "log_detection" => Self::LOG_DETECTION,
            "workload_security" => Self::WORKLOAD_SECURITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
