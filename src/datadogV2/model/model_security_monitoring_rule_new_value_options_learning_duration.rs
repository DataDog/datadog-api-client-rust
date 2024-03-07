// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningDuration {
    ZERO_DAYS,
    ONE_DAY,
    SEVEN_DAYS,
}

impl Serialize for SecurityMonitoringRuleNewValueOptionsLearningDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            Self::ZERO_DAYS => 0,
            Self::ONE_DAY => 1,
            Self::SEVEN_DAYS => 7,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleNewValueOptionsLearningDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::ZERO_DAYS,
            1 => Self::ONE_DAY,
            7 => Self::SEVEN_DAYS,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleNewValueOptionsLearningDuration: {}",
                    s
                )))
            }
        })
    }
}
