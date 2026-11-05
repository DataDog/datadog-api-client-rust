// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewaySupportedOperation {
    ANTHROPIC_MESSAGES,
    ANTHROPIC_COUNT_TOKENS,
    OPENAI_CHAT_COMPLETIONS,
    OPENAI_RESPONSES,
    OPENAI_WEB_SEARCH,
    OPENAI_EMBEDDINGS,
    GEMINI_GENERATE_CONTENT,
    GEMINI_RAW_PREDICTIONS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewaySupportedOperation {
    fn to_string(&self) -> String {
        match self {
            Self::ANTHROPIC_MESSAGES => String::from("ANTHROPIC_MESSAGES"),
            Self::ANTHROPIC_COUNT_TOKENS => String::from("ANTHROPIC_COUNT_TOKENS"),
            Self::OPENAI_CHAT_COMPLETIONS => String::from("OPENAI_CHAT_COMPLETIONS"),
            Self::OPENAI_RESPONSES => String::from("OPENAI_RESPONSES"),
            Self::OPENAI_WEB_SEARCH => String::from("OPENAI_WEB_SEARCH"),
            Self::OPENAI_EMBEDDINGS => String::from("OPENAI_EMBEDDINGS"),
            Self::GEMINI_GENERATE_CONTENT => String::from("GEMINI_GENERATE_CONTENT"),
            Self::GEMINI_RAW_PREDICTIONS => String::from("GEMINI_RAW_PREDICTIONS"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewaySupportedOperation {
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

impl<'de> Deserialize<'de> for AiGatewaySupportedOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ANTHROPIC_MESSAGES" => Self::ANTHROPIC_MESSAGES,
            "ANTHROPIC_COUNT_TOKENS" => Self::ANTHROPIC_COUNT_TOKENS,
            "OPENAI_CHAT_COMPLETIONS" => Self::OPENAI_CHAT_COMPLETIONS,
            "OPENAI_RESPONSES" => Self::OPENAI_RESPONSES,
            "OPENAI_WEB_SEARCH" => Self::OPENAI_WEB_SEARCH,
            "OPENAI_EMBEDDINGS" => Self::OPENAI_EMBEDDINGS,
            "GEMINI_GENERATE_CONTENT" => Self::GEMINI_GENERATE_CONTENT,
            "GEMINI_RAW_PREDICTIONS" => Self::GEMINI_RAW_PREDICTIONS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
