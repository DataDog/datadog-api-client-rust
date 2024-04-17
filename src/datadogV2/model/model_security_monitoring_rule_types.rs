// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleTypes {
    APPLICATION_SECURITY,
    LOG_DETECTION,
    CLOUD_CONFIGURATION,
    INFRASTRUCTURE_CONFIGURATION,
    WORKLOAD_SECURITY,
    SIGNAL_CORRELATION,
    VULNERABILITY_MANAGEMENT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringRuleTypes {
    fn to_string(&self) -> String {
        match self {
            Self::APPLICATION_SECURITY => String::from("application_security"),
            Self::LOG_DETECTION => String::from("log_detection"),
            Self::CLOUD_CONFIGURATION => String::from("cloud_configuration"),
            Self::INFRASTRUCTURE_CONFIGURATION => String::from("infrastructure_configuration"),
            Self::WORKLOAD_SECURITY => String::from("workload_security"),
            Self::SIGNAL_CORRELATION => String::from("signal_correlation"),
            Self::VULNERABILITY_MANAGEMENT => String::from("vulnerability_management"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleTypes {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "application_security" => Self::APPLICATION_SECURITY,
            "log_detection" => Self::LOG_DETECTION,
            "cloud_configuration" => Self::CLOUD_CONFIGURATION,
            "infrastructure_configuration" => Self::INFRASTRUCTURE_CONFIGURATION,
            "workload_security" => Self::WORKLOAD_SECURITY,
            "signal_correlation" => Self::SIGNAL_CORRELATION,
            "vulnerability_management" => Self::VULNERABILITY_MANAGEMENT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
