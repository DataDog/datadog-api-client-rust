// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsBanditPosteriorUnavailableReason {
    NO_DATA,
    INSUFFICIENT_DATA,
    ZERO_VARIANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsBanditPosteriorUnavailableReason {
    fn to_string(&self) -> String {
        match self {
            Self::NO_DATA => String::from("no_data"),
            Self::INSUFFICIENT_DATA => String::from("insufficient_data"),
            Self::ZERO_VARIANCE => String::from("zero_variance"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsBanditPosteriorUnavailableReason {
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

impl<'de> Deserialize<'de> for ExperimentsBanditPosteriorUnavailableReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "no_data" => Self::NO_DATA,
            "insufficient_data" => Self::INSUFFICIENT_DATA,
            "zero_variance" => Self::ZERO_VARIANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
