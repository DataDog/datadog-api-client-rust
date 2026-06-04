// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to handle JWT authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBasicAuthJWT {
    /// Standard JWT claims to automatically inject.
    #[serde(rename = "addClaims")]
    pub add_claims: Option<crate::datadogV1::model::SyntheticsBasicAuthJWTAddClaims>,
    /// Algorithm to use for the JWT authentication.
    #[serde(rename = "algorithm")]
    pub algorithm: crate::datadogV1::model::SyntheticsBasicAuthJWTAlgorithm,
    /// Token time-to-live in seconds.
    #[serde(rename = "expiresIn")]
    pub expires_in: Option<i64>,
    /// Custom JWT header as a JSON string.
    #[serde(rename = "header")]
    pub header: Option<String>,
    /// JWT claims as a JSON string.
    #[serde(rename = "payload")]
    pub payload: String,
    /// Signing key for the JWT authentication. Use the shared secret for `HS256`
    /// or the private key (PEM format) for `RS256` and `ES256`.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Prefix added before the token in the `Authorization` header. Defaults to `Bearer`.
    #[serde(rename = "tokenPrefix")]
    pub token_prefix: Option<String>,
    /// The type of authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBasicAuthJWTType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBasicAuthJWT {
    pub fn new(
        algorithm: crate::datadogV1::model::SyntheticsBasicAuthJWTAlgorithm,
        payload: String,
        secret: String,
        type_: crate::datadogV1::model::SyntheticsBasicAuthJWTType,
    ) -> SyntheticsBasicAuthJWT {
        SyntheticsBasicAuthJWT {
            add_claims: None,
            algorithm,
            expires_in: None,
            header: None,
            payload,
            secret,
            token_prefix: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn add_claims(
        mut self,
        value: crate::datadogV1::model::SyntheticsBasicAuthJWTAddClaims,
    ) -> Self {
        self.add_claims = Some(value);
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    pub fn header(mut self, value: String) -> Self {
        self.header = Some(value);
        self
    }

    pub fn token_prefix(mut self, value: String) -> Self {
        self.token_prefix = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthJWT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthJWTVisitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthJWTVisitor {
            type Value = SyntheticsBasicAuthJWT;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut add_claims: Option<
                    crate::datadogV1::model::SyntheticsBasicAuthJWTAddClaims,
                > = None;
                let mut algorithm: Option<
                    crate::datadogV1::model::SyntheticsBasicAuthJWTAlgorithm,
                > = None;
                let mut expires_in: Option<i64> = None;
                let mut header: Option<String> = None;
                let mut payload: Option<String> = None;
                let mut secret: Option<String> = None;
                let mut token_prefix: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBasicAuthJWTType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "addClaims" => {
                            if v.is_null() {
                                continue;
                            }
                            add_claims = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "algorithm" => {
                            algorithm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _algorithm) = algorithm {
                                match _algorithm {
                                    crate::datadogV1::model::SyntheticsBasicAuthJWTAlgorithm::UnparsedObject(_algorithm) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "expiresIn" => {
                            if v.is_null() {
                                continue;
                            }
                            expires_in = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "header" => {
                            if v.is_null() {
                                continue;
                            }
                            header = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "payload" => {
                            payload = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret" => {
                            secret = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tokenPrefix" => {
                            if v.is_null() {
                                continue;
                            }
                            token_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBasicAuthJWTType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let algorithm = algorithm.ok_or_else(|| M::Error::missing_field("algorithm"))?;
                let payload = payload.ok_or_else(|| M::Error::missing_field("payload"))?;
                let secret = secret.ok_or_else(|| M::Error::missing_field("secret"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsBasicAuthJWT {
                    add_claims,
                    algorithm,
                    expires_in,
                    header,
                    payload,
                    secret,
                    token_prefix,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthJWTVisitor)
    }
}
