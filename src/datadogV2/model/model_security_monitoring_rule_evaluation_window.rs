// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SecurityMonitoringRuleEvaluationWindow {
    ZERO_MINUTES,
    ONE_MINUTE,
    FIVE_MINUTES,
    TEN_MINUTES,
    FIFTEEN_MINUTES,
    THIRTY_MINUTES,
    ONE_HOUR,
    TWO_HOURS,
}

impl Serialize for SecurityMonitoringRuleEvaluationWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SecurityMonitoringRuleEvaluationWindow::ZERO_MINUTES => 0,
            SecurityMonitoringRuleEvaluationWindow::ONE_MINUTE => 60,
            SecurityMonitoringRuleEvaluationWindow::FIVE_MINUTES => 300,
            SecurityMonitoringRuleEvaluationWindow::TEN_MINUTES => 600,
            SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES => 900,
            SecurityMonitoringRuleEvaluationWindow::THIRTY_MINUTES => 1800,
            SecurityMonitoringRuleEvaluationWindow::ONE_HOUR => 3600,
            SecurityMonitoringRuleEvaluationWindow::TWO_HOURS => 7200,
        })
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleEvaluationWindow {
    fn deserialize<D>(deserializer: D) -> Result<SecurityMonitoringRuleEvaluationWindow, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => SecurityMonitoringRuleEvaluationWindow::ZERO_MINUTES,
            60 => SecurityMonitoringRuleEvaluationWindow::ONE_MINUTE,
            300 => SecurityMonitoringRuleEvaluationWindow::FIVE_MINUTES,
            600 => SecurityMonitoringRuleEvaluationWindow::TEN_MINUTES,
            900 => SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES,
            1800 => SecurityMonitoringRuleEvaluationWindow::THIRTY_MINUTES,
            3600 => SecurityMonitoringRuleEvaluationWindow::ONE_HOUR,
            7200 => SecurityMonitoringRuleEvaluationWindow::TWO_HOURS,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SecurityMonitoringRuleEvaluationWindow: {}",
                    s
                )))
            }
        })
    }
}
