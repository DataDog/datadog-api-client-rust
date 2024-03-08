// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SensitiveDataScannerProduct {
    LOGS,
    RUM,
    EVENTS,
    APM,
}

impl ToString for SensitiveDataScannerProduct {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::RUM => String::from("rum"),
            Self::EVENTS => String::from("events"),
            Self::APM => String::from("apm"),
        }
    }
}

impl Serialize for SensitiveDataScannerProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerProduct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "logs" => Self::LOGS,
            "rum" => Self::RUM,
            "events" => Self::EVENTS,
            "apm" => Self::APM,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
