// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SecurityMonitoringRuleMaxSignalDuration {
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
    TWELVE_HOURS,
    ONE_DAY,
}

impl ToString for SecurityMonitoringRuleMaxSignalDuration {
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
            Self::TWELVE_HOURS => String::from("43200"),
            Self::ONE_DAY => String::from("86400"),
        }
    }
}
impl Serialize for SecurityMonitoringRuleMaxSignalDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SecurityMonitoringRuleMaxSignalDuration::ZERO_MINUTES => 0,
            SecurityMonitoringRuleMaxSignalDuration::ONE_MINUTE => 60,
            SecurityMonitoringRuleMaxSignalDuration::FIVE_MINUTES => 300,
            SecurityMonitoringRuleMaxSignalDuration::TEN_MINUTES => 600,
            SecurityMonitoringRuleMaxSignalDuration::FIFTEEN_MINUTES => 900,
            SecurityMonitoringRuleMaxSignalDuration::THIRTY_MINUTES => 1800,
            SecurityMonitoringRuleMaxSignalDuration::ONE_HOUR => 3600,
            SecurityMonitoringRuleMaxSignalDuration::TWO_HOURS => 7200,
            SecurityMonitoringRuleMaxSignalDuration::THREE_HOURS => 10800,
            SecurityMonitoringRuleMaxSignalDuration::SIX_HOURS => 21600,
            SecurityMonitoringRuleMaxSignalDuration::TWELVE_HOURS => 43200,
            SecurityMonitoringRuleMaxSignalDuration::ONE_DAY => 86400,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleMaxSignalDuration {
    fn deserialize<D>(deserializer: D) -> Result<SecurityMonitoringRuleMaxSignalDuration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => SecurityMonitoringRuleMaxSignalDuration::ZERO_MINUTES,
            60 => SecurityMonitoringRuleMaxSignalDuration::ONE_MINUTE,
            300 => SecurityMonitoringRuleMaxSignalDuration::FIVE_MINUTES,
            600 => SecurityMonitoringRuleMaxSignalDuration::TEN_MINUTES,
            900 => SecurityMonitoringRuleMaxSignalDuration::FIFTEEN_MINUTES,
            1800 => SecurityMonitoringRuleMaxSignalDuration::THIRTY_MINUTES,
            3600 => SecurityMonitoringRuleMaxSignalDuration::ONE_HOUR,
            7200 => SecurityMonitoringRuleMaxSignalDuration::TWO_HOURS,
            10800 => SecurityMonitoringRuleMaxSignalDuration::THREE_HOURS,
            21600 => SecurityMonitoringRuleMaxSignalDuration::SIX_HOURS,
            43200 => SecurityMonitoringRuleMaxSignalDuration::TWELVE_HOURS,
            86400 => SecurityMonitoringRuleMaxSignalDuration::ONE_DAY,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleMaxSignalDuration: {}",
                    s
                )))
            }
        })
    }
}
