// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsLocalVariableParsingOptionsType {
    GRPC_MESSAGE,
    GRPC_METADATA,
    HTTP_BODY,
    HTTP_HEADER,
    HTTP_STATUS_CODE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsLocalVariableParsingOptionsType {
    fn to_string(&self) -> String {
        match self {
            Self::GRPC_MESSAGE => String::from("grpc_message"),
            Self::GRPC_METADATA => String::from("grpc_metadata"),
            Self::HTTP_BODY => String::from("http_body"),
            Self::HTTP_HEADER => String::from("http_header"),
            Self::HTTP_STATUS_CODE => String::from("http_status_code"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsLocalVariableParsingOptionsType {
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

impl<'de> Deserialize<'de> for SyntheticsLocalVariableParsingOptionsType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "grpc_message" => Self::GRPC_MESSAGE,
            "grpc_metadata" => Self::GRPC_METADATA,
            "http_body" => Self::HTTP_BODY,
            "http_header" => Self::HTTP_HEADER,
            "http_status_code" => Self::HTTP_STATUS_CODE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
