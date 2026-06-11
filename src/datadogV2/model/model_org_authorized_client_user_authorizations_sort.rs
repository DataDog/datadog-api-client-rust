// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrgAuthorizedClientUserAuthorizationsSort {
    USER_NAME,
    USER_EMAIL,
    OAUTH2_CLIENT_NAME,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OrgAuthorizedClientUserAuthorizationsSort {
    fn to_string(&self) -> String {
        match self {
            Self::USER_NAME => String::from("user.name"),
            Self::USER_EMAIL => String::from("user.email"),
            Self::OAUTH2_CLIENT_NAME => String::from("oauth2_client.name"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OrgAuthorizedClientUserAuthorizationsSort {
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

impl<'de> Deserialize<'de> for OrgAuthorizedClientUserAuthorizationsSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "user.name" => Self::USER_NAME,
            "user.email" => Self::USER_EMAIL,
            "oauth2_client.name" => Self::OAUTH2_CLIENT_NAME,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
