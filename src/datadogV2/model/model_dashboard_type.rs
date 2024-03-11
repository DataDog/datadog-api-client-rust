// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DashboardType {
    CUSTOM_TIMEBOARD,
    CUSTOM_SCREENBOARD,
    INTEGRATION_SCREENBOARD,
    INTEGRATION_TIMEBOARD,
    HOST_TIMEBOARD,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for DashboardType {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM_TIMEBOARD => String::from("custom_timeboard"),
            Self::CUSTOM_SCREENBOARD => String::from("custom_screenboard"),
            Self::INTEGRATION_SCREENBOARD => String::from("integration_screenboard"),
            Self::INTEGRATION_TIMEBOARD => String::from("integration_timeboard"),
            Self::HOST_TIMEBOARD => String::from("host_timeboard"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DashboardType {
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

impl<'de> Deserialize<'de> for DashboardType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom_timeboard" => Self::CUSTOM_TIMEBOARD,
            "custom_screenboard" => Self::CUSTOM_SCREENBOARD,
            "integration_screenboard" => Self::INTEGRATION_SCREENBOARD,
            "integration_timeboard" => Self::INTEGRATION_TIMEBOARD,
            "host_timeboard" => Self::HOST_TIMEBOARD,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
