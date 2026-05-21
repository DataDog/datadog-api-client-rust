// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LLMObsIntegrationName {
    OPENAI,
    AMAZON_BEDROCK,
    ANTHROPIC,
    AZURE_OPENAI,
    VERTEX_AI,
    LLMPROXY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for LLMObsIntegrationName {
    fn to_string(&self) -> String {
        match self {
            Self::OPENAI => String::from("openai"),
            Self::AMAZON_BEDROCK => String::from("amazon_bedrock"),
            Self::ANTHROPIC => String::from("anthropic"),
            Self::AZURE_OPENAI => String::from("azure_openai"),
            Self::VERTEX_AI => String::from("vertex_ai"),
            Self::LLMPROXY => String::from("llmproxy"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for LLMObsIntegrationName {
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

impl<'de> Deserialize<'de> for LLMObsIntegrationName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "openai" => Self::OPENAI,
            "amazon_bedrock" => Self::AMAZON_BEDROCK,
            "anthropic" => Self::ANTHROPIC,
            "azure_openai" => Self::AZURE_OPENAI,
            "vertex_ai" => Self::VERTEX_AI,
            "llmproxy" => Self::LLMPROXY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
