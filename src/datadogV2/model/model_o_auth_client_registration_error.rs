// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Error payload returned by OAuth2 dynamic client registration as defined by RFC 7591.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OAuthClientRegistrationError {
    /// Single ASCII error code per RFC 7591, such as `invalid_request` or `invalid_client_metadata`.
    #[serde(rename = "error")]
    pub error: String,
    /// Human-readable description of the error.
    #[serde(rename = "error_description")]
    pub error_description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OAuthClientRegistrationError {
    pub fn new(error: String, error_description: String) -> OAuthClientRegistrationError {
        OAuthClientRegistrationError {
            error,
            error_description,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for OAuthClientRegistrationError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OAuthClientRegistrationErrorVisitor;
        impl<'a> Visitor<'a> for OAuthClientRegistrationErrorVisitor {
            type Value = OAuthClientRegistrationError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<String> = None;
                let mut error_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_description" => {
                            error_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error = error.ok_or_else(|| M::Error::missing_field("error"))?;
                let error_description = error_description
                    .ok_or_else(|| M::Error::missing_field("error_description"))?;

                let content = OAuthClientRegistrationError {
                    error,
                    error_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OAuthClientRegistrationErrorVisitor)
    }
}
