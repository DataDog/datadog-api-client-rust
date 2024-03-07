// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
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

impl ToString for SecurityMonitoringRuleKeepAlive {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO_MINUTES => String::from("0"),
            Self::ONE_MINUTE => String::from("60"),
            Self::FIVE_MINUTES => String::from("300"),
            Self::TEN_MINUTES => String::from("600"),
            Self::FIFTEEN_MINUTES => String::from("900"),
            Self::THIRTY_MINUTES => String::from("1800"),
            Self::ONE_HOUR => String::from("3600"),
            Self::TWO_HOURS => String::from("7200"),
            Self::THREE_HOURS => String::from("10800"),
            Self::SIX_HOURS => String::from("21600"),
        }
    }
}
impl Serialize for SecurityMonitoringRuleKeepAlive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SecurityMonitoringRuleKeepAlive::ZERO_MINUTES => 0,
            SecurityMonitoringRuleKeepAlive::ONE_MINUTE => 60,
            SecurityMonitoringRuleKeepAlive::FIVE_MINUTES => 300,
            SecurityMonitoringRuleKeepAlive::TEN_MINUTES => 600,
            SecurityMonitoringRuleKeepAlive::FIFTEEN_MINUTES => 900,
            SecurityMonitoringRuleKeepAlive::THIRTY_MINUTES => 1800,
            SecurityMonitoringRuleKeepAlive::ONE_HOUR => 3600,
            SecurityMonitoringRuleKeepAlive::TWO_HOURS => 7200,
            SecurityMonitoringRuleKeepAlive::THREE_HOURS => 10800,
            SecurityMonitoringRuleKeepAlive::SIX_HOURS => 21600,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleKeepAlive {
    fn deserialize<D>(deserializer: D) -> Result<SecurityMonitoringRuleKeepAlive, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => SecurityMonitoringRuleKeepAlive::ZERO_MINUTES,
            60 => SecurityMonitoringRuleKeepAlive::ONE_MINUTE,
            300 => SecurityMonitoringRuleKeepAlive::FIVE_MINUTES,
            600 => SecurityMonitoringRuleKeepAlive::TEN_MINUTES,
            900 => SecurityMonitoringRuleKeepAlive::FIFTEEN_MINUTES,
            1800 => SecurityMonitoringRuleKeepAlive::THIRTY_MINUTES,
            3600 => SecurityMonitoringRuleKeepAlive::ONE_HOUR,
            7200 => SecurityMonitoringRuleKeepAlive::TWO_HOURS,
            10800 => SecurityMonitoringRuleKeepAlive::THREE_HOURS,
            21600 => SecurityMonitoringRuleKeepAlive::SIX_HOURS,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleKeepAlive: {}",
                    s
                )))
            }
        })
    }
}
