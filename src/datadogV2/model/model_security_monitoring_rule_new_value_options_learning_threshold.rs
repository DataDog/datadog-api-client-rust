// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[non_exhaustive]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    ZERO_OCCURRENCES = 0,
    ONE_OCCURRENCE = 1,
}

impl ToString for SecurityMonitoringRuleNewValueOptionsLearningThreshold {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO_OCCURRENCES => String::from("0"),
            Self::ONE_OCCURRENCE => String::from("1"),
        }
    }
}
