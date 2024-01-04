// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum SecurityMonitoringRuleNewValueOptionsForgetAfter {
    ONE_DAY = 1,
    TWO_DAYS = 2,
    ONE_WEEK = 7,
    TWO_WEEKS = 14,
    THREE_WEEKS = 21,
    FOUR_WEEKS = 28,
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
