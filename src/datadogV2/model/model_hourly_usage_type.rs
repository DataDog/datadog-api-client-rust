// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HourlyUsageType {
    APP_SEC_HOST_COUNT,
    OBSERVABILITY_PIPELINES_BYTES_PROCESSSED,
    LAMBDA_TRACED_INVOCATIONS_COUNT,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for HourlyUsageType {
    fn to_string(&self) -> String {
        match self {
            Self::APP_SEC_HOST_COUNT => String::from("app_sec_host_count"),
            Self::OBSERVABILITY_PIPELINES_BYTES_PROCESSSED => {
                String::from("observability_pipelines_bytes_processed")
            }
            Self::LAMBDA_TRACED_INVOCATIONS_COUNT => {
                String::from("lambda_traced_invocations_count")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for HourlyUsageType {
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

impl<'de> Deserialize<'de> for HourlyUsageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "app_sec_host_count" => Self::APP_SEC_HOST_COUNT,
            "observability_pipelines_bytes_processed" => {
                Self::OBSERVABILITY_PIPELINES_BYTES_PROCESSSED
            }
            "lambda_traced_invocations_count" => Self::LAMBDA_TRACED_INVOCATIONS_COUNT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
