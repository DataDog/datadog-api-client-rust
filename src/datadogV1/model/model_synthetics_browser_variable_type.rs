// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsBrowserVariableType {
    ELEMENT,
    EMAIL,
    GLOBAL,
    JAVASCRIPT,
    TEXT,
}

impl ToString for SyntheticsBrowserVariableType {
    fn to_string(&self) -> String {
        match self {
            Self::ELEMENT => String::from("element"),
            Self::EMAIL => String::from("email"),
            Self::GLOBAL => String::from("global"),
            Self::JAVASCRIPT => String::from("javascript"),
            Self::TEXT => String::from("text"),
        }
    }
}

impl Serialize for SyntheticsBrowserVariableType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserVariableType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "element" => Self::ELEMENT,
            "email" => Self::EMAIL,
            "global" => Self::GLOBAL,
            "javascript" => Self::JAVASCRIPT,
            "text" => Self::TEXT,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
