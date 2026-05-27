// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OAuthOidcScope {
    OPENID,
    PROFILE,
    EMAIL,
    OFFLINE_ACCESS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OAuthOidcScope {
    fn to_string(&self) -> String {
        match self {
            Self::OPENID => String::from("openid"),
            Self::PROFILE => String::from("profile"),
            Self::EMAIL => String::from("email"),
            Self::OFFLINE_ACCESS => String::from("offline_access"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OAuthOidcScope {
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

impl<'de> Deserialize<'de> for OAuthOidcScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "openid" => Self::OPENID,
            "profile" => Self::PROFILE,
            "email" => Self::EMAIL,
            "offline_access" => Self::OFFLINE_ACCESS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
