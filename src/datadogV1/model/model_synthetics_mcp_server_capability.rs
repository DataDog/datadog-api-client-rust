// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsMCPServerCapability {
    COMPLETIONS,
    EXPERIMENTAL,
    LOGGING,
    PROMPTS,
    RESOURCES,
    TOOLS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsMCPServerCapability {
    fn to_string(&self) -> String {
        match self {
            Self::COMPLETIONS => String::from("completions"),
            Self::EXPERIMENTAL => String::from("experimental"),
            Self::LOGGING => String::from("logging"),
            Self::PROMPTS => String::from("prompts"),
            Self::RESOURCES => String::from("resources"),
            Self::TOOLS => String::from("tools"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsMCPServerCapability {
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

impl<'de> Deserialize<'de> for SyntheticsMCPServerCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "completions" => Self::COMPLETIONS,
            "experimental" => Self::EXPERIMENTAL,
            "logging" => Self::LOGGING,
            "prompts" => Self::PROMPTS,
            "resources" => Self::RESOURCES,
            "tools" => Self::TOOLS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
