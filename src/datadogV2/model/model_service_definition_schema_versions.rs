// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceDefinitionSchemaVersions {
    V1,
    V2,
    V2_1,
    V2_2,
}

impl ToString for ServiceDefinitionSchemaVersions {
    fn to_string(&self) -> String {
        match self {
            Self::V1 => String::from("v1"),
            Self::V2 => String::from("v2"),
            Self::V2_1 => String::from("v2.1"),
            Self::V2_2 => String::from("v2.2"),
        }
    }
}

impl Serialize for ServiceDefinitionSchemaVersions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionSchemaVersions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "v1" => Self::V1,
            "v2" => Self::V2,
            "v2.1" => Self::V2_1,
            "v2.2" => Self::V2_2,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
