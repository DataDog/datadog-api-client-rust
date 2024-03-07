// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceCheckStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

impl Serialize for ServiceCheckStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            ServiceCheckStatus::OK => 0,
            ServiceCheckStatus::WARNING => 1,
            ServiceCheckStatus::CRITICAL => 2,
            ServiceCheckStatus::UNKNOWN => 3,
        })
    }
}

impl<'de> Deserialize<'de> for ServiceCheckStatus {
    fn deserialize<D>(deserializer: D) -> Result<ServiceCheckStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => ServiceCheckStatus::OK,
            1 => ServiceCheckStatus::WARNING,
            2 => ServiceCheckStatus::CRITICAL,
            3 => ServiceCheckStatus::UNKNOWN,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for ServiceCheckStatus: {}",
                    s
                )))
            }
        })
    }
}
