// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayMetadataCaptureItem {
    TOOL_USAGE,
    TOOL_OUTPUT_SIZE,
    TOOL_NAMES,
    CONVERSATION_SHAPE,
    INPUT_SIZE_BY_ROLE,
    AVAILABLE_TOOLS,
    GENERATION_SETTINGS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayMetadataCaptureItem {
    fn to_string(&self) -> String {
        match self {
            Self::TOOL_USAGE => String::from("tool_usage"),
            Self::TOOL_OUTPUT_SIZE => String::from("tool_output_size"),
            Self::TOOL_NAMES => String::from("tool_names"),
            Self::CONVERSATION_SHAPE => String::from("conversation_shape"),
            Self::INPUT_SIZE_BY_ROLE => String::from("input_size_by_role"),
            Self::AVAILABLE_TOOLS => String::from("available_tools"),
            Self::GENERATION_SETTINGS => String::from("generation_settings"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayMetadataCaptureItem {
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

impl<'de> Deserialize<'de> for AiGatewayMetadataCaptureItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "tool_usage" => Self::TOOL_USAGE,
            "tool_output_size" => Self::TOOL_OUTPUT_SIZE,
            "tool_names" => Self::TOOL_NAMES,
            "conversation_shape" => Self::CONVERSATION_SHAPE,
            "input_size_by_role" => Self::INPUT_SIZE_BY_ROLE,
            "available_tools" => Self::AVAILABLE_TOOLS,
            "generation_settings" => Self::GENERATION_SETTINGS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
