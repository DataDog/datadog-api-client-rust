// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PersonalAccessTokensSort {
    NAME_ASCENDING,
    NAME_DESCENDING,
    CREATED_AT_ASCENDING,
    CREATED_AT_DESCENDING,
    EXPIRES_AT_ASCENDING,
    EXPIRES_AT_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for PersonalAccessTokensSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME_ASCENDING => String::from("name"),
            Self::NAME_DESCENDING => String::from("-name"),
            Self::CREATED_AT_ASCENDING => String::from("created_at"),
            Self::CREATED_AT_DESCENDING => String::from("-created_at"),
            Self::EXPIRES_AT_ASCENDING => String::from("expires_at"),
            Self::EXPIRES_AT_DESCENDING => String::from("-expires_at"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for PersonalAccessTokensSort {
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

impl<'de> Deserialize<'de> for PersonalAccessTokensSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME_ASCENDING,
            "-name" => Self::NAME_DESCENDING,
            "created_at" => Self::CREATED_AT_ASCENDING,
            "-created_at" => Self::CREATED_AT_DESCENDING,
            "expires_at" => Self::EXPIRES_AT_ASCENDING,
            "-expires_at" => Self::EXPIRES_AT_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
