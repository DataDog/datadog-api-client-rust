// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceCheckStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

impl Serialize for ServiceCheckStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OK => serializer.serialize_i32(0),
            Self::WARNING => serializer.serialize_i32(1),
            Self::CRITICAL => serializer.serialize_i32(2),
            Self::UNKNOWN => serializer.serialize_i32(3),
        }
    }
}

impl<'de> Deserialize<'de> for ServiceCheckStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::OK,
            1 => Self::WARNING,
            2 => Self::CRITICAL,
            3 => Self::UNKNOWN,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
