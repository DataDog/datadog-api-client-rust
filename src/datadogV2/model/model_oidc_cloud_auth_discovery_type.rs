// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OIDCCloudAuthDiscoveryType {
    OIDC_DISCOVERY,
    DIRECT_JWKS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OIDCCloudAuthDiscoveryType {
    fn to_string(&self) -> String {
        match self {
            Self::OIDC_DISCOVERY => String::from("oidc_discovery"),
            Self::DIRECT_JWKS => String::from("direct_jwks"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OIDCCloudAuthDiscoveryType {
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

impl<'de> Deserialize<'de> for OIDCCloudAuthDiscoveryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "oidc_discovery" => Self::OIDC_DISCOVERY,
            "direct_jwks" => Self::DIRECT_JWKS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
