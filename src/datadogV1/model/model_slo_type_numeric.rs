// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SLOTypeNumeric {
    MONITOR,
    METRIC,
}

impl ToString for SLOTypeNumeric {
    fn to_string(&self) -> String {
        match self {
            Self::MONITOR => String::from("0"),
            Self::METRIC => String::from("1"),
        }
    }
}
impl Serialize for SLOTypeNumeric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            SLOTypeNumeric::MONITOR => 0,
            SLOTypeNumeric::METRIC => 1,
        })
    }
}

impl<'de> Deserialize<'de> for SLOTypeNumeric {
    fn deserialize<D>(deserializer: D) -> Result<SLOTypeNumeric, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => SLOTypeNumeric::MONITOR,
            1 => SLOTypeNumeric::METRIC,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SLOTypeNumeric: {}",
                    s
                )))
            }
        })
    }
}
