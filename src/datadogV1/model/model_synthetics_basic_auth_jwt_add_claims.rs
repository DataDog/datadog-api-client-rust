// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Standard JWT claims to automatically inject.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBasicAuthJWTAddClaims {
    /// Whether to inject the `exp` (expiration) claim.
    #[serde(rename = "exp")]
    pub exp: Option<bool>,
    /// Whether to inject the `iat` (issued at) claim.
    #[serde(rename = "iat")]
    pub iat: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBasicAuthJWTAddClaims {
    pub fn new() -> SyntheticsBasicAuthJWTAddClaims {
        SyntheticsBasicAuthJWTAddClaims {
            exp: None,
            iat: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exp(mut self, value: bool) -> Self {
        self.exp = Some(value);
        self
    }

    pub fn iat(mut self, value: bool) -> Self {
        self.iat = Some(value);
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

impl Default for SyntheticsBasicAuthJWTAddClaims {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthJWTAddClaims {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthJWTAddClaimsVisitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthJWTAddClaimsVisitor {
            type Value = SyntheticsBasicAuthJWTAddClaims;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exp: Option<bool> = None;
                let mut iat: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exp" => {
                            if v.is_null() {
                                continue;
                            }
                            exp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iat" => {
                            if v.is_null() {
                                continue;
                            }
                            iat = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsBasicAuthJWTAddClaims {
                    exp,
                    iat,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthJWTAddClaimsVisitor)
    }
}
