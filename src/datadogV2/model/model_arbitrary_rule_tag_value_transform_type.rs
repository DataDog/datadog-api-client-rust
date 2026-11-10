// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArbitraryRuleTagValueTransformType {
    STRING_REPLACE,
    REGEX_EXTRACT,
    REGEX_REPLACE,
    LOOKUP_MAP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ArbitraryRuleTagValueTransformType {
    fn to_string(&self) -> String {
        match self {
            Self::STRING_REPLACE => String::from("string_replace"),
            Self::REGEX_EXTRACT => String::from("regex_extract"),
            Self::REGEX_REPLACE => String::from("regex_replace"),
            Self::LOOKUP_MAP => String::from("lookup_map"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ArbitraryRuleTagValueTransformType {
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

impl<'de> Deserialize<'de> for ArbitraryRuleTagValueTransformType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "string_replace" => Self::STRING_REPLACE,
            "regex_extract" => Self::REGEX_EXTRACT,
            "regex_replace" => Self::REGEX_REPLACE,
            "lookup_map" => Self::LOOKUP_MAP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
