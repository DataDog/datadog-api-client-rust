// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleTypeRead {
    LOG_DETECTION,
    INFRASTRUCTURE_CONFIGURATION,
    WORKLOAD_SECURITY,
    CLOUD_CONFIGURATION,
    APPLICATION_SECURITY,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SecurityMonitoringRuleTypeRead {
    fn to_string(&self) -> String {
        match self {
            Self::LOG_DETECTION => String::from("log_detection"),
            Self::INFRASTRUCTURE_CONFIGURATION => String::from("infrastructure_configuration"),
            Self::WORKLOAD_SECURITY => String::from("workload_security"),
            Self::CLOUD_CONFIGURATION => String::from("cloud_configuration"),
            Self::APPLICATION_SECURITY => String::from("application_security"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleTypeRead {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleTypeRead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "log_detection" => Self::LOG_DETECTION,
            "infrastructure_configuration" => Self::INFRASTRUCTURE_CONFIGURATION,
            "workload_security" => Self::WORKLOAD_SECURITY,
            "cloud_configuration" => Self::CLOUD_CONFIGURATION,
            "application_security" => Self::APPLICATION_SECURITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
