// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsBanditResultsState {
    WAITING_FOR_FIRST_REFRESH,
    WARMUP,
    OPTIMIZING,
    STATS_UNAVAILABLE,
    ENDED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsBanditResultsState {
    fn to_string(&self) -> String {
        match self {
            Self::WAITING_FOR_FIRST_REFRESH => String::from("WAITING_FOR_FIRST_REFRESH"),
            Self::WARMUP => String::from("WARMUP"),
            Self::OPTIMIZING => String::from("OPTIMIZING"),
            Self::STATS_UNAVAILABLE => String::from("STATS_UNAVAILABLE"),
            Self::ENDED => String::from("ENDED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsBanditResultsState {
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

impl<'de> Deserialize<'de> for ExperimentsBanditResultsState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "WAITING_FOR_FIRST_REFRESH" => Self::WAITING_FOR_FIRST_REFRESH,
            "WARMUP" => Self::WARMUP,
            "OPTIMIZING" => Self::OPTIMIZING,
            "STATS_UNAVAILABLE" => Self::STATS_UNAVAILABLE,
            "ENDED" => Self::ENDED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
