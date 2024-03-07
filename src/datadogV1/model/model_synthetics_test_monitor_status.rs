// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(i64)]
pub enum SyntheticsTestMonitorStatus {
    UNTRIGGERED,
    TRIGGERED,
    NO_DATA,
}

impl ToString for SyntheticsTestMonitorStatus {
    fn to_string(&self) -> String {
        match self {
            Self::UNTRIGGERED => String::from("0"),
            Self::TRIGGERED => String::from("1"),
            Self::NO_DATA => String::from("2"),
        }
    }
}
impl Serialize for SyntheticsTestMonitorStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SyntheticsTestMonitorStatus::UNTRIGGERED => 0,
            SyntheticsTestMonitorStatus::TRIGGERED => 1,
            SyntheticsTestMonitorStatus::NO_DATA => 2,
        })
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestMonitorStatus {
    fn deserialize<D>(deserializer: D) -> Result<SyntheticsTestMonitorStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => SyntheticsTestMonitorStatus::UNTRIGGERED,
            1 => SyntheticsTestMonitorStatus::TRIGGERED,
            2 => SyntheticsTestMonitorStatus::NO_DATA,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsTestMonitorStatus: {}",
                    s
                )))
            }
        })
    }
}
