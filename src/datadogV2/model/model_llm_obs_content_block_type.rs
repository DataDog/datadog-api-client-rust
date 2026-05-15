// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LLMObsContentBlockType {
    MARKDOWN,
    HEADER,
    TEXT,
    JSON,
    IMAGE,
    WIDGET,
    LLMOBS_TRACE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for LLMObsContentBlockType {
    fn to_string(&self) -> String {
        match self {
            Self::MARKDOWN => String::from("markdown"),
            Self::HEADER => String::from("header"),
            Self::TEXT => String::from("text"),
            Self::JSON => String::from("json"),
            Self::IMAGE => String::from("image"),
            Self::WIDGET => String::from("widget"),
            Self::LLMOBS_TRACE => String::from("llmobs_trace"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for LLMObsContentBlockType {
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

impl<'de> Deserialize<'de> for LLMObsContentBlockType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "markdown" => Self::MARKDOWN,
            "header" => Self::HEADER,
            "text" => Self::TEXT,
            "json" => Self::JSON,
            "image" => Self::IMAGE,
            "widget" => Self::WIDGET,
            "llmobs_trace" => Self::LLMOBS_TRACE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
