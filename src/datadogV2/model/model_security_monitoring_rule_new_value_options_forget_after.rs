// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SecurityMonitoringRuleNewValueOptionsForgetAfter {
    ONE_DAY,
    TWO_DAYS,
    ONE_WEEK,
    TWO_WEEKS,
    THREE_WEEKS,
    FOUR_WEEKS,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsForgetAfter {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_DAY => String::from("1"),
            Self::TWO_DAYS => String::from("2"),
            Self::ONE_WEEK => String::from("7"),
            Self::TWO_WEEKS => String::from("14"),
            Self::THREE_WEEKS => String::from("21"),
            Self::FOUR_WEEKS => String::from("28"),
        }
    }
}
impl Serialize for SecurityMonitoringRuleNewValueOptionsForgetAfter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SecurityMonitoringRuleNewValueOptionsForgetAfter::ONE_DAY => 1,
            SecurityMonitoringRuleNewValueOptionsForgetAfter::TWO_DAYS => 2,
            SecurityMonitoringRuleNewValueOptionsForgetAfter::ONE_WEEK => 7,
            SecurityMonitoringRuleNewValueOptionsForgetAfter::TWO_WEEKS => 14,
            SecurityMonitoringRuleNewValueOptionsForgetAfter::THREE_WEEKS => 21,
            SecurityMonitoringRuleNewValueOptionsForgetAfter::FOUR_WEEKS => 28,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleNewValueOptionsForgetAfter {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<SecurityMonitoringRuleNewValueOptionsForgetAfter, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            1 => SecurityMonitoringRuleNewValueOptionsForgetAfter::ONE_DAY,
            2 => SecurityMonitoringRuleNewValueOptionsForgetAfter::TWO_DAYS,
            7 => SecurityMonitoringRuleNewValueOptionsForgetAfter::ONE_WEEK,
            14 => SecurityMonitoringRuleNewValueOptionsForgetAfter::TWO_WEEKS,
            21 => SecurityMonitoringRuleNewValueOptionsForgetAfter::THREE_WEEKS,
            28 => SecurityMonitoringRuleNewValueOptionsForgetAfter::FOUR_WEEKS,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleNewValueOptionsForgetAfter: {}",
                    s
                )))
            }
        })
    }
}
