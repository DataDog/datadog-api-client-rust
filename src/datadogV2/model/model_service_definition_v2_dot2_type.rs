// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceDefinitionV2Dot2Type {
    WEB,
    DB,
    CACHE,
    FUNCTION,
    BROSWER,
    MOBILE,
    CUSTOM,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for ServiceDefinitionV2Dot2Type {
    fn to_string(&self) -> String {
        match self {
            Self::WEB => String::from("web"),
            Self::DB => String::from("db"),
            Self::CACHE => String::from("cache"),
            Self::FUNCTION => String::from("function"),
            Self::BROSWER => String::from("browser"),
            Self::MOBILE => String::from("mobile"),
            Self::CUSTOM => String::from("custom"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ServiceDefinitionV2Dot2Type {
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

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot2Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "web" => Self::WEB,
            "db" => Self::DB,
            "cache" => Self::CACHE,
            "function" => Self::FUNCTION,
            "browser" => Self::BROSWER,
            "mobile" => Self::MOBILE,
            "custom" => Self::CUSTOM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
