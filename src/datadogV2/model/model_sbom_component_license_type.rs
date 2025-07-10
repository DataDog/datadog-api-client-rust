// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SBOMComponentLicenseType {
    NETWORK_STRONG_COPYLEFT,
    NON_STANDARD_COPYLEFT,
    OTHER_NON_FREE,
    OTHER_NON_STANDARD,
    PERMISSIVE,
    PUBLIC_DOMAIN,
    STRONG_COPYLEFT,
    WEAK_COPYLEFT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SBOMComponentLicenseType {
    fn to_string(&self) -> String {
        match self {
            Self::NETWORK_STRONG_COPYLEFT => String::from("network_strong_copyleft"),
            Self::NON_STANDARD_COPYLEFT => String::from("non_standard_copyleft"),
            Self::OTHER_NON_FREE => String::from("other_non_free"),
            Self::OTHER_NON_STANDARD => String::from("other_non_standard"),
            Self::PERMISSIVE => String::from("permissive"),
            Self::PUBLIC_DOMAIN => String::from("public_domain"),
            Self::STRONG_COPYLEFT => String::from("strong_copyleft"),
            Self::WEAK_COPYLEFT => String::from("weak_copyleft"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SBOMComponentLicenseType {
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

impl<'de> Deserialize<'de> for SBOMComponentLicenseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "network_strong_copyleft" => Self::NETWORK_STRONG_COPYLEFT,
            "non_standard_copyleft" => Self::NON_STANDARD_COPYLEFT,
            "other_non_free" => Self::OTHER_NON_FREE,
            "other_non_standard" => Self::OTHER_NON_STANDARD,
            "permissive" => Self::PERMISSIVE,
            "public_domain" => Self::PUBLIC_DOMAIN,
            "strong_copyleft" => Self::STRONG_COPYLEFT,
            "weak_copyleft" => Self::WEAK_COPYLEFT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
