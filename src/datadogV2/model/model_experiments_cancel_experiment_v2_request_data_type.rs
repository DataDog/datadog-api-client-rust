// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsCancelExperimentV2RequestDataType {
    CANCEL_EXPERIMENT_REQUEST,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsCancelExperimentV2RequestDataType {
    fn to_string(&self) -> String {
        match self {
            Self::CANCEL_EXPERIMENT_REQUEST => String::from("cancel-experiment-request"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsCancelExperimentV2RequestDataType {
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

impl<'de> Deserialize<'de> for ExperimentsCancelExperimentV2RequestDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "cancel-experiment-request" => Self::CANCEL_EXPERIMENT_REQUEST,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
