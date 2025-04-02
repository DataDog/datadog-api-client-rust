// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringStandardDataSource {
    LOGS,
    AUDIT,
    APP_SEC_SPANS,
    SPANS,
    SECURITY_RUNTIME,
    NETWORK,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringStandardDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::AUDIT => String::from("audit"),
            Self::APP_SEC_SPANS => String::from("app_sec_spans"),
            Self::SPANS => String::from("spans"),
            Self::SECURITY_RUNTIME => String::from("security_runtime"),
            Self::NETWORK => String::from("network"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringStandardDataSource {
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

impl<'de> Deserialize<'de> for SecurityMonitoringStandardDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "logs" => Self::LOGS,
            "audit" => Self::AUDIT,
            "app_sec_spans" => Self::APP_SEC_SPANS,
            "spans" => Self::SPANS,
            "security_runtime" => Self::SECURITY_RUNTIME,
            "network" => Self::NETWORK,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
