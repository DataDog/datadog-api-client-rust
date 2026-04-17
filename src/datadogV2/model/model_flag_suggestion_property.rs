// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlagSuggestionProperty {
    FLAG,
    FLAG_NAME,
    FLAG_DESCRIPTION,
    JSON_SCHEMA,
    DISTRIBUTION_CHANNEL,
    VARIANT,
    VARIANT_NAME,
    VARIANT_VALUE,
    ALLOCATIONS,
    ROLLOUT,
    ENVIRONMENT_STATUS,
    DEFAULT_VARIANT,
    OVERRIDE_VARIANT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FlagSuggestionProperty {
    fn to_string(&self) -> String {
        match self {
            Self::FLAG => String::from("FLAG"),
            Self::FLAG_NAME => String::from("FLAG_NAME"),
            Self::FLAG_DESCRIPTION => String::from("FLAG_DESCRIPTION"),
            Self::JSON_SCHEMA => String::from("JSON_SCHEMA"),
            Self::DISTRIBUTION_CHANNEL => String::from("DISTRIBUTION_CHANNEL"),
            Self::VARIANT => String::from("VARIANT"),
            Self::VARIANT_NAME => String::from("VARIANT_NAME"),
            Self::VARIANT_VALUE => String::from("VARIANT_VALUE"),
            Self::ALLOCATIONS => String::from("ALLOCATIONS"),
            Self::ROLLOUT => String::from("ROLLOUT"),
            Self::ENVIRONMENT_STATUS => String::from("ENVIRONMENT_STATUS"),
            Self::DEFAULT_VARIANT => String::from("DEFAULT_VARIANT"),
            Self::OVERRIDE_VARIANT => String::from("OVERRIDE_VARIANT"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FlagSuggestionProperty {
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

impl<'de> Deserialize<'de> for FlagSuggestionProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "FLAG" => Self::FLAG,
            "FLAG_NAME" => Self::FLAG_NAME,
            "FLAG_DESCRIPTION" => Self::FLAG_DESCRIPTION,
            "JSON_SCHEMA" => Self::JSON_SCHEMA,
            "DISTRIBUTION_CHANNEL" => Self::DISTRIBUTION_CHANNEL,
            "VARIANT" => Self::VARIANT,
            "VARIANT_NAME" => Self::VARIANT_NAME,
            "VARIANT_VALUE" => Self::VARIANT_VALUE,
            "ALLOCATIONS" => Self::ALLOCATIONS,
            "ROLLOUT" => Self::ROLLOUT,
            "ENVIRONMENT_STATUS" => Self::ENVIRONMENT_STATUS,
            "DEFAULT_VARIANT" => Self::DEFAULT_VARIANT,
            "OVERRIDE_VARIANT" => Self::OVERRIDE_VARIANT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
