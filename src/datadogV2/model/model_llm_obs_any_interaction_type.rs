// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LLMObsAnyInteractionType {
    TRACE,
    EXPERIMENT_TRACE,
    SESSION,
    DISPLAY_BLOCK,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for LLMObsAnyInteractionType {
    fn to_string(&self) -> String {
        match self {
            Self::TRACE => String::from("trace"),
            Self::EXPERIMENT_TRACE => String::from("experiment_trace"),
            Self::SESSION => String::from("session"),
            Self::DISPLAY_BLOCK => String::from("display_block"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for LLMObsAnyInteractionType {
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

impl<'de> Deserialize<'de> for LLMObsAnyInteractionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "trace" => Self::TRACE,
            "experiment_trace" => Self::EXPERIMENT_TRACE,
            "session" => Self::SESSION,
            "display_block" => Self::DISPLAY_BLOCK,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
