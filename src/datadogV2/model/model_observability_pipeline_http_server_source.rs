// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `http_server` source collects logs over HTTP POST from external services.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineHttpServerSource {
    /// Name of the environment variable or secret that holds the listen address for the HTTP server.
    #[serde(rename = "address_key")]
    pub address_key: Option<String>,
    /// HTTP authentication method.
    #[serde(rename = "auth_strategy")]
    pub auth_strategy: crate::datadogV2::model::ObservabilityPipelineHttpServerSourceAuthStrategy,
    /// Name of the environment variable or secret that holds a custom header value (used with custom auth strategies).
    #[serde(rename = "custom_key")]
    pub custom_key: Option<String>,
    /// The decoding format used to interpret incoming logs.
    #[serde(rename = "decoding")]
    pub decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
    /// Unique ID for the HTTP server source.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the environment variable or secret that holds the password (used when `auth_strategy` is `plain`).
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The source type. The value should always be `http_server`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineHttpServerSourceType,
    /// Name of the environment variable or secret that holds the username (used when `auth_strategy` is `plain`).
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineHttpServerSource {
    pub fn new(
        auth_strategy: crate::datadogV2::model::ObservabilityPipelineHttpServerSourceAuthStrategy,
        decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
        id: String,
        type_: crate::datadogV2::model::ObservabilityPipelineHttpServerSourceType,
    ) -> ObservabilityPipelineHttpServerSource {
        ObservabilityPipelineHttpServerSource {
            address_key: None,
            auth_strategy,
            custom_key: None,
            decoding,
            id,
            password_key: None,
            tls: None,
            type_,
            username_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn address_key(mut self, value: String) -> Self {
        self.address_key = Some(value);
        self
    }

    pub fn custom_key(mut self, value: String) -> Self {
        self.custom_key = Some(value);
        self
    }

    pub fn password_key(mut self, value: String) -> Self {
        self.password_key = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
        self
    }

    pub fn username_key(mut self, value: String) -> Self {
        self.username_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineHttpServerSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineHttpServerSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineHttpServerSourceVisitor {
            type Value = ObservabilityPipelineHttpServerSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut address_key: Option<String> = None;
                let mut auth_strategy: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpServerSourceAuthStrategy,
                > = None;
                let mut custom_key: Option<String> = None;
                let mut decoding: Option<crate::datadogV2::model::ObservabilityPipelineDecoding> =
                    None;
                let mut id: Option<String> = None;
                let mut password_key: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpServerSourceType,
                > = None;
                let mut username_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "address_key" => {
                            if v.is_null() {
                                continue;
                            }
                            address_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auth_strategy" => {
                            auth_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_strategy) = auth_strategy {
                                match _auth_strategy {
                                    crate::datadogV2::model::ObservabilityPipelineHttpServerSourceAuthStrategy::UnparsedObject(_auth_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "custom_key" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decoding" => {
                            decoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _decoding) = decoding {
                                match _decoding {
                                    crate::datadogV2::model::ObservabilityPipelineDecoding::UnparsedObject(_decoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "password_key" => {
                            if v.is_null() {
                                continue;
                            }
                            password_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineHttpServerSourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "username_key" => {
                            if v.is_null() {
                                continue;
                            }
                            username_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_strategy =
                    auth_strategy.ok_or_else(|| M::Error::missing_field("auth_strategy"))?;
                let decoding = decoding.ok_or_else(|| M::Error::missing_field("decoding"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineHttpServerSource {
                    address_key,
                    auth_strategy,
                    custom_key,
                    decoding,
                    id,
                    password_key,
                    tls,
                    type_,
                    username_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineHttpServerSourceVisitor)
    }
}
