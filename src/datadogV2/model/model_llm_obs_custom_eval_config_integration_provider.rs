// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LLMObsCustomEvalConfigIntegrationProvider {
    OPENAI,
    AMAZON_BEDROCK,
    ANTHROPIC,
    AZURE_OPENAI,
    VERTEX_AI,
    LLM_PROXY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for LLMObsCustomEvalConfigIntegrationProvider {
    fn to_string(&self) -> String {
        match self {
            Self::OPENAI => String::from("openai"),
            Self::AMAZON_BEDROCK => String::from("amazon-bedrock"),
            Self::ANTHROPIC => String::from("anthropic"),
            Self::AZURE_OPENAI => String::from("azure-openai"),
            Self::VERTEX_AI => String::from("vertex-ai"),
            Self::LLM_PROXY => String::from("llm-proxy"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for LLMObsCustomEvalConfigIntegrationProvider {
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

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigIntegrationProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "openai" => Self::OPENAI,
            "amazon-bedrock" => Self::AMAZON_BEDROCK,
            "anthropic" => Self::ANTHROPIC,
            "azure-openai" => Self::AZURE_OPENAI,
            "vertex-ai" => Self::VERTEX_AI,
            "llm-proxy" => Self::LLM_PROXY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
