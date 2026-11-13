// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsBanditArmDataStatus {
    USABLE,
    INSUFFICIENT_DATA,
    ZERO_VARIANCE,
    POOLED_WITHIN_VARIANCE,
    POOLED_TOTAL_VARIANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsBanditArmDataStatus {
    fn to_string(&self) -> String {
        match self {
            Self::USABLE => String::from("usable"),
            Self::INSUFFICIENT_DATA => String::from("insufficient_data"),
            Self::ZERO_VARIANCE => String::from("zero_variance"),
            Self::POOLED_WITHIN_VARIANCE => String::from("pooled_within_variance"),
            Self::POOLED_TOTAL_VARIANCE => String::from("pooled_total_variance"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsBanditArmDataStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ExperimentsBanditArmDataStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "usable" => Self::USABLE,
            "insufficient_data" => Self::INSUFFICIENT_DATA,
            "zero_variance" => Self::ZERO_VARIANCE,
            "pooled_within_variance" => Self::POOLED_WITHIN_VARIANCE,
            "pooled_total_variance" => Self::POOLED_TOTAL_VARIANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
