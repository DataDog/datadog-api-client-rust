// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsTestRequestBodyType {
    TEXT_PLAIN,
    APPLICATION_JSON,
    TEXT_XML,
    TEXT_HTML,
    APPLICATION_X_WWW_FORM_URLENCODED,
    GRAPHQL,
    APPLICATION_OCTET_STREAM,
    MULTIPART_FORM_DATA,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsTestRequestBodyType {
    fn to_string(&self) -> String {
        match self {
            Self::TEXT_PLAIN => String::from("text/plain"),
            Self::APPLICATION_JSON => String::from("application/json"),
            Self::TEXT_XML => String::from("text/xml"),
            Self::TEXT_HTML => String::from("text/html"),
            Self::APPLICATION_X_WWW_FORM_URLENCODED => {
                String::from("application/x-www-form-urlencoded")
            }
            Self::GRAPHQL => String::from("graphql"),
            Self::APPLICATION_OCTET_STREAM => String::from("application/octet-stream"),
            Self::MULTIPART_FORM_DATA => String::from("multipart/form-data"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsTestRequestBodyType {
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

impl<'de> Deserialize<'de> for SyntheticsTestRequestBodyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "text/plain" => Self::TEXT_PLAIN,
            "application/json" => Self::APPLICATION_JSON,
            "text/xml" => Self::TEXT_XML,
            "text/html" => Self::TEXT_HTML,
            "application/x-www-form-urlencoded" => Self::APPLICATION_X_WWW_FORM_URLENCODED,
            "graphql" => Self::GRAPHQL,
            "application/octet-stream" => Self::APPLICATION_OCTET_STREAM,
            "multipart/form-data" => Self::MULTIPART_FORM_DATA,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
