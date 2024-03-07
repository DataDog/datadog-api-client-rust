// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleDetectionMethod {
    #[serde(rename = "threshold")]
    THRESHOLD,
    #[serde(rename = "new_value")]
    NEW_VALUE,
    #[serde(rename = "anomaly_detection")]
    ANOMALY_DETECTION,
    #[serde(rename = "impossible_travel")]
    IMPOSSIBLE_TRAVEL,
    #[serde(rename = "hardcoded")]
    HARDCODED,
    #[serde(rename = "third_party")]
    THIRD_PARTY,
}

impl ToString for SecurityMonitoringRuleDetectionMethod {
    fn to_string(&self) -> String {
        match self {
            Self::THRESHOLD => String::from("threshold"),
            Self::NEW_VALUE => String::from("new_value"),
            Self::ANOMALY_DETECTION => String::from("anomaly_detection"),
            Self::IMPOSSIBLE_TRAVEL => String::from("impossible_travel"),
            Self::HARDCODED => String::from("hardcoded"),
            Self::THIRD_PARTY => String::from("third_party"),
        }
    }
}
