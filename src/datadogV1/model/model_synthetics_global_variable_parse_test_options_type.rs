// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsGlobalVariableParseTestOptionsType {
    HTTP_BODY,
    HTTP_HEADER,
    LOCAL_VARIABLE,
}

impl ToString for SyntheticsGlobalVariableParseTestOptionsType {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP_BODY => String::from("http_body"),
            Self::HTTP_HEADER => String::from("http_header"),
            Self::LOCAL_VARIABLE => String::from("local_variable"),
        }
    }
}

impl Serialize for SyntheticsGlobalVariableParseTestOptionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsGlobalVariableParseTestOptionsType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "http_body" => Self::HTTP_BODY,
            "http_header" => Self::HTTP_HEADER,
            "local_variable" => Self::LOCAL_VARIABLE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
