// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[non_exhaustive]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum SecurityMonitoringRuleMaxSignalDuration {
    ZERO_MINUTES = 0,
    ONE_MINUTE = 60,
    FIVE_MINUTES = 300,
    TEN_MINUTES = 600,
    FIFTEEN_MINUTES = 900,
    THIRTY_MINUTES = 1800,
    ONE_HOUR = 3600,
    TWO_HOURS = 7200,
    THREE_HOURS = 10800,
    SIX_HOURS = 21600,
    TWELVE_HOURS = 43200,
    ONE_DAY = 86400,
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
