// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RolesSort {
    NAME_ASCENDING,
    NAME_DESCENDING,
    MODIFIED_AT_ASCENDING,
    MODIFIED_AT_DESCENDING,
    USER_COUNT_ASCENDING,
    USER_COUNT_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for RolesSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME_ASCENDING => String::from("name"),
            Self::NAME_DESCENDING => String::from("-name"),
            Self::MODIFIED_AT_ASCENDING => String::from("modified_at"),
            Self::MODIFIED_AT_DESCENDING => String::from("-modified_at"),
            Self::USER_COUNT_ASCENDING => String::from("user_count"),
            Self::USER_COUNT_DESCENDING => String::from("-user_count"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RolesSort {
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

impl<'de> Deserialize<'de> for RolesSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME_ASCENDING,
            "-name" => Self::NAME_DESCENDING,
            "modified_at" => Self::MODIFIED_AT_ASCENDING,
            "-modified_at" => Self::MODIFIED_AT_DESCENDING,
            "user_count" => Self::USER_COUNT_ASCENDING,
            "-user_count" => Self::USER_COUNT_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
