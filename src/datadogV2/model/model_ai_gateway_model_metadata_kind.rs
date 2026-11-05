// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayModelMetadataKind {
    LANGUAGE,
    EMBEDDING,
    IMAGE,
    AUDIO,
    DECISION,
    RERANKING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayModelMetadataKind {
    fn to_string(&self) -> String {
        match self {
            Self::LANGUAGE => String::from("language"),
            Self::EMBEDDING => String::from("embedding"),
            Self::IMAGE => String::from("image"),
            Self::AUDIO => String::from("audio"),
            Self::DECISION => String::from("decision"),
            Self::RERANKING => String::from("reranking"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayModelMetadataKind {
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

impl<'de> Deserialize<'de> for AiGatewayModelMetadataKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "language" => Self::LANGUAGE,
            "embedding" => Self::EMBEDDING,
            "image" => Self::IMAGE,
            "audio" => Self::AUDIO,
            "decision" => Self::DECISION,
            "reranking" => Self::RERANKING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
