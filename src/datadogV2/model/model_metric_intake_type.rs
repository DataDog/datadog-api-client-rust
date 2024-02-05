// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde_repr::{Deserialize_repr, Serialize_repr};

#[non_exhaustive]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum MetricIntakeType {
    UNSPECIFIED = 0,
    COUNT = 1,
    RATE = 2,
    GAUGE = 3,
}

impl ToString for MetricIntakeType {
    fn to_string(&self) -> String {
        match self {
            Self::UNSPECIFIED => String::from("0"),
            Self::COUNT => String::from("1"),
            Self::RATE => String::from("2"),
            Self::GAUGE => String::from("3"),
        }
    }
}
