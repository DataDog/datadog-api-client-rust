// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningDuration {
    ZERO_DAYS = 0,
    ONE_DAY = 1,
    SEVEN_DAYS = 7,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsLearningDuration {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO_DAYS => String::from("0"),
            Self::ONE_DAY => String::from("1"),
            Self::SEVEN_DAYS => String::from("7"),
        }
    }
}
