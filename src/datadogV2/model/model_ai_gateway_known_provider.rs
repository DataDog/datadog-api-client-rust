// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayKnownProvider {
    CUSTOM,
    ANTHROPIC,
    OPENAI,
    AZURE_OPENAI,
    BEDROCK,
    GEMINI,
    BASETEN,
    FIREWORKS,
    TOGETHER,
    TYPESAFE,
    AZURE_FOUNDRY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayKnownProvider {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM => String::from("custom"),
            Self::ANTHROPIC => String::from("anthropic"),
            Self::OPENAI => String::from("openai"),
            Self::AZURE_OPENAI => String::from("azure_openai"),
            Self::BEDROCK => String::from("bedrock"),
            Self::GEMINI => String::from("gemini"),
            Self::BASETEN => String::from("baseten"),
            Self::FIREWORKS => String::from("fireworks"),
            Self::TOGETHER => String::from("together"),
            Self::TYPESAFE => String::from("typesafe"),
            Self::AZURE_FOUNDRY => String::from("azure_foundry"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayKnownProvider {
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

impl<'de> Deserialize<'de> for AiGatewayKnownProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => Self::CUSTOM,
            "anthropic" => Self::ANTHROPIC,
            "openai" => Self::OPENAI,
            "azure_openai" => Self::AZURE_OPENAI,
            "bedrock" => Self::BEDROCK,
            "gemini" => Self::GEMINI,
            "baseten" => Self::BASETEN,
            "fireworks" => Self::FIREWORKS,
            "together" => Self::TOGETHER,
            "typesafe" => Self::TYPESAFE,
            "azure_foundry" => Self::AZURE_FOUNDRY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
