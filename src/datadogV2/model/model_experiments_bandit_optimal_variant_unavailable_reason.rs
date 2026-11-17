// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsBanditOptimalVariantUnavailableReason {
    NO_RESULTS_YET,
    WARMUP,
    NOT_ENOUGH_DATA,
    NO_DIFFERENCE,
    STATS_UNAVAILABLE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsBanditOptimalVariantUnavailableReason {
    fn to_string(&self) -> String {
        match self {
            Self::NO_RESULTS_YET => String::from("NO_RESULTS_YET"),
            Self::WARMUP => String::from("WARMUP"),
            Self::NOT_ENOUGH_DATA => String::from("NOT_ENOUGH_DATA"),
            Self::NO_DIFFERENCE => String::from("NO_DIFFERENCE"),
            Self::STATS_UNAVAILABLE => String::from("STATS_UNAVAILABLE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsBanditOptimalVariantUnavailableReason {
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

impl<'de> Deserialize<'de> for ExperimentsBanditOptimalVariantUnavailableReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "NO_RESULTS_YET" => Self::NO_RESULTS_YET,
            "WARMUP" => Self::WARMUP,
            "NOT_ENOUGH_DATA" => Self::NOT_ENOUGH_DATA,
            "NO_DIFFERENCE" => Self::NO_DIFFERENCE,
            "STATS_UNAVAILABLE" => Self::STATS_UNAVAILABLE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
