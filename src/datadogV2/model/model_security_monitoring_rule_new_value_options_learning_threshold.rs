// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    ZERO_OCCURRENCES,
    ONE_OCCURRENCE,
}

impl Serialize for SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SecurityMonitoringRuleNewValueOptionsLearningThreshold::ZERO_OCCURRENCES => 0,
            SecurityMonitoringRuleNewValueOptionsLearningThreshold::ONE_OCCURRENCE => 1,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<SecurityMonitoringRuleNewValueOptionsLearningThreshold, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => SecurityMonitoringRuleNewValueOptionsLearningThreshold::ZERO_OCCURRENCES,
            1 => SecurityMonitoringRuleNewValueOptionsLearningThreshold::ONE_OCCURRENCE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleNewValueOptionsLearningThreshold: {}",
                    s
                )))
            }
        })
    }
}
