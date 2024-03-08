// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationKeysSort {
    CREATED_AT_ASCENDING,
    CREATED_AT_DESCENDING,
    LAST4_ASCENDING,
    LAST4_DESCENDING,
    NAME_ASCENDING,
    NAME_DESCENDING,
}

impl ToString for ApplicationKeysSort {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED_AT_ASCENDING => String::from("created_at"),
            Self::CREATED_AT_DESCENDING => String::from("-created_at"),
            Self::LAST4_ASCENDING => String::from("last4"),
            Self::LAST4_DESCENDING => String::from("-last4"),
            Self::NAME_ASCENDING => String::from("name"),
            Self::NAME_DESCENDING => String::from("-name"),
        }
    }
}

impl Serialize for ApplicationKeysSort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ApplicationKeysSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created_at" => Self::CREATED_AT_ASCENDING,
            "-created_at" => Self::CREATED_AT_DESCENDING,
            "last4" => Self::LAST4_ASCENDING,
            "-last4" => Self::LAST4_DESCENDING,
            "name" => Self::NAME_ASCENDING,
            "-name" => Self::NAME_DESCENDING,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
