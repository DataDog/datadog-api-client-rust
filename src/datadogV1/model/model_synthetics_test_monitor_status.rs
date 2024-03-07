// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsTestMonitorStatus {
    UNTRIGGERED,
    TRIGGERED,
    NO_DATA,
}

impl Serialize for SyntheticsTestMonitorStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(match self {
            Self::UNTRIGGERED => 0,
            Self::TRIGGERED => 1,
            Self::NO_DATA => 2,
        })
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestMonitorStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i64 = i64::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::UNTRIGGERED,
            1 => Self::TRIGGERED,
            2 => Self::NO_DATA,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsTestMonitorStatus: {}",
                    s
                )))
            }
        })
    }
}
