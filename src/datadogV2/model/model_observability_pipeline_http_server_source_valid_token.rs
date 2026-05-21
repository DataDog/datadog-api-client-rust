// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An accepted token used to authenticate incoming HTTP server requests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineHttpServerSourceValidToken {
    /// Indicates whether this token is currently accepted. Disabled tokens are rejected without
    /// being removed from the configuration.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// An optional metadata field that is attached to every event authenticated by the
    /// associated token. Both `key` and `value` must match `^[A-Za-z0-9_]+$`.
    #[serde(rename = "field_to_add")]
    pub field_to_add:
        Option<crate::datadogV2::model::ObservabilityPipelineSourceValidTokenFieldToAdd>,
    /// Specifies where the worker extracts the token from in the incoming HTTP request.
    /// This can be either a built-in location (`path` or `address`) or an HTTP header object.
    #[serde(rename = "path_to_token")]
    pub path_to_token:
        Option<crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToToken>,
    /// Name of the environment variable or secret that holds the expected token value.
    #[serde(rename = "token_key")]
    pub token_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineHttpServerSourceValidToken {
    pub fn new(token_key: String) -> ObservabilityPipelineHttpServerSourceValidToken {
        ObservabilityPipelineHttpServerSourceValidToken {
            enabled: None,
            field_to_add: None,
            path_to_token: None,
            token_key,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn field_to_add(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineSourceValidTokenFieldToAdd,
    ) -> Self {
        self.field_to_add = Some(value);
        self
    }

    pub fn path_to_token(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToToken,
    ) -> Self {
        self.path_to_token = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineHttpServerSourceValidToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineHttpServerSourceValidTokenVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineHttpServerSourceValidTokenVisitor {
            type Value = ObservabilityPipelineHttpServerSourceValidToken;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut field_to_add: Option<
                    crate::datadogV2::model::ObservabilityPipelineSourceValidTokenFieldToAdd,
                > = None;
                let mut path_to_token: Option<crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToToken> = None;
                let mut token_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_to_add" => {
                            if v.is_null() {
                                continue;
                            }
                            field_to_add =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path_to_token" => {
                            if v.is_null() {
                                continue;
                            }
                            path_to_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _path_to_token) = path_to_token {
                                match _path_to_token {
                                    crate::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToToken::UnparsedObject(_path_to_token) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "token_key" => {
                            token_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let token_key = token_key.ok_or_else(|| M::Error::missing_field("token_key"))?;

                let content = ObservabilityPipelineHttpServerSourceValidToken {
                    enabled,
                    field_to_add,
                    path_to_token,
                    token_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineHttpServerSourceValidTokenVisitor)
    }
}
