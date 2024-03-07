// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleKeepAlive {
    ZERO_MINUTES,
    ONE_MINUTE,
    FIVE_MINUTES,
    TEN_MINUTES,
    FIFTEEN_MINUTES,
    THIRTY_MINUTES,
    ONE_HOUR,
    TWO_HOURS,
    THREE_HOURS,
    SIX_HOURS,
}

impl Serialize for SecurityMonitoringRuleKeepAlive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            Self::ZERO_MINUTES => 0,
            Self::ONE_MINUTE => 60,
            Self::FIVE_MINUTES => 300,
            Self::TEN_MINUTES => 600,
            Self::FIFTEEN_MINUTES => 900,
            Self::THIRTY_MINUTES => 1800,
            Self::ONE_HOUR => 3600,
            Self::TWO_HOURS => 7200,
            Self::THREE_HOURS => 10800,
            Self::SIX_HOURS => 21600,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleKeepAlive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::ZERO_MINUTES,
            60 => Self::ONE_MINUTE,
            300 => Self::FIVE_MINUTES,
            600 => Self::TEN_MINUTES,
            900 => Self::FIFTEEN_MINUTES,
            1800 => Self::THIRTY_MINUTES,
            3600 => Self::ONE_HOUR,
            7200 => Self::TWO_HOURS,
            10800 => Self::THREE_HOURS,
            21600 => Self::SIX_HOURS,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleKeepAlive: {}",
                    s
                )))
            }
        })
    }
}
