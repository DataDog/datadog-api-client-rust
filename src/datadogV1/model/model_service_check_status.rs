// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[non_exhaustive]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ServiceCheckStatus {
    OK = 0,
    WARNING = 1,
    CRITICAL = 2,
    UNKNOWN = 3,
}

impl ToString for ServiceCheckStatus {
    fn to_string(&self) -> String {
        match self {
            Self::OK => String::from("0"),
            Self::WARNING => String::from("1"),
            Self::CRITICAL => String::from("2"),
            Self::UNKNOWN => String::from("3"),
        }
    }
}
