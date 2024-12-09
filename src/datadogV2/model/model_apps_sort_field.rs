// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppsSortField {
    NAME,
    CREATED_AT,
    UPDATED_AT,
    USER_NAME,
    NAME_DESC,
    CREATED_AT_DESC,
    UPDATED_AT_DESC,
    USER_NAME_DESC,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AppsSortField {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::CREATED_AT => String::from("created_at"),
            Self::UPDATED_AT => String::from("updated_at"),
            Self::USER_NAME => String::from("user_name"),
            Self::NAME_DESC => String::from("-name"),
            Self::CREATED_AT_DESC => String::from("-created_at"),
            Self::UPDATED_AT_DESC => String::from("-updated_at"),
            Self::USER_NAME_DESC => String::from("-user_name"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AppsSortField {
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

impl<'de> Deserialize<'de> for AppsSortField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "created_at" => Self::CREATED_AT,
            "updated_at" => Self::UPDATED_AT,
            "user_name" => Self::USER_NAME,
            "-name" => Self::NAME_DESC,
            "-created_at" => Self::CREATED_AT_DESC,
            "-updated_at" => Self::UPDATED_AT_DESC,
            "-user_name" => Self::USER_NAME_DESC,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
