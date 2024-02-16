// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[non_exhaustive]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i64)]
pub enum SyntheticsTestMonitorStatus {
    UNTRIGGERED = 0,
    TRIGGERED = 1,
    NO_DATA = 2,
}

impl ToString for SyntheticsTestMonitorStatus {
    fn to_string(&self) -> String {
        match self {
            Self::UNTRIGGERED => String::from("0"),
            Self::TRIGGERED => String::from("1"),
            Self::NO_DATA => String::from("2"),
        }
    }
}
