// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeatureFlagChangeEventProperty {
    FLAG,
    FLAG_NAME,
    FLAG_DESCRIPTION,
    JSON_SCHEMA,
    DISTRIBUTION_CHANNEL,
    REQUIRE_APPROVAL,
    VARIANT,
    VARIANT_NAME,
    VARIANT_VALUE,
    ALLOCATIONS,
    ROLLOUT,
    ENVIRONMENT_STATUS,
    DEFAULT_VARIANT,
    OVERRIDE_VARIANT,
    STALENESS_STATUS,
    SAVED_FILTER,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FeatureFlagChangeEventProperty {
    fn to_string(&self) -> String {
        match self {
            Self::FLAG => String::from("FLAG"),
            Self::FLAG_NAME => String::from("FLAG_NAME"),
            Self::FLAG_DESCRIPTION => String::from("FLAG_DESCRIPTION"),
            Self::JSON_SCHEMA => String::from("JSON_SCHEMA"),
            Self::DISTRIBUTION_CHANNEL => String::from("DISTRIBUTION_CHANNEL"),
            Self::REQUIRE_APPROVAL => String::from("REQUIRE_APPROVAL"),
            Self::VARIANT => String::from("VARIANT"),
            Self::VARIANT_NAME => String::from("VARIANT_NAME"),
            Self::VARIANT_VALUE => String::from("VARIANT_VALUE"),
            Self::ALLOCATIONS => String::from("ALLOCATIONS"),
            Self::ROLLOUT => String::from("ROLLOUT"),
            Self::ENVIRONMENT_STATUS => String::from("ENVIRONMENT_STATUS"),
            Self::DEFAULT_VARIANT => String::from("DEFAULT_VARIANT"),
            Self::OVERRIDE_VARIANT => String::from("OVERRIDE_VARIANT"),
            Self::STALENESS_STATUS => String::from("STALENESS_STATUS"),
            Self::SAVED_FILTER => String::from("SAVED_FILTER"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FeatureFlagChangeEventProperty {
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

impl<'de> Deserialize<'de> for FeatureFlagChangeEventProperty {
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
            "REQUIRE_APPROVAL" => Self::REQUIRE_APPROVAL,
            "VARIANT" => Self::VARIANT,
            "VARIANT_NAME" => Self::VARIANT_NAME,
            "VARIANT_VALUE" => Self::VARIANT_VALUE,
            "ALLOCATIONS" => Self::ALLOCATIONS,
            "ROLLOUT" => Self::ROLLOUT,
            "ENVIRONMENT_STATUS" => Self::ENVIRONMENT_STATUS,
            "DEFAULT_VARIANT" => Self::DEFAULT_VARIANT,
            "OVERRIDE_VARIANT" => Self::OVERRIDE_VARIANT,
            "STALENESS_STATUS" => Self::STALENESS_STATUS,
            "SAVED_FILTER" => Self::SAVED_FILTER,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
