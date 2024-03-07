// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetricIntakeType {
    UNSPECIFIED,
    COUNT,
    RATE,
    GAUGE,
}

impl Serialize for MetricIntakeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            MetricIntakeType::UNSPECIFIED => 0,
            MetricIntakeType::COUNT => 1,
            MetricIntakeType::RATE => 2,
            MetricIntakeType::GAUGE => 3,
        })
    }
}

impl<'de> Deserialize<'de> for MetricIntakeType {
    fn deserialize<D>(deserializer: D) -> Result<MetricIntakeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => MetricIntakeType::UNSPECIFIED,
            1 => MetricIntakeType::COUNT,
            2 => MetricIntakeType::RATE,
            3 => MetricIntakeType::GAUGE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for MetricIntakeType: {}",
                    s
                )))
            }
        })
    }
}
