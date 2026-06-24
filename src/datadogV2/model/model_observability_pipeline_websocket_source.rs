// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `websocket` source ingests logs from a WebSocket server using the `<ws://`> or `<wss://`> protocol.
///
/// **Supported pipeline types:** logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineWebsocketSource {
    /// Authentication strategy for the WebSocket source connection.
    #[serde(rename = "auth_strategy")]
    pub auth_strategy: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceAuthStrategy,
    /// Name of the environment variable or secret that holds the custom authorization header value. Used when `auth_strategy` is `custom`.
    #[serde(rename = "custom_key")]
    pub custom_key: Option<String>,
    /// The decoding format used to interpret incoming logs.
    #[serde(rename = "decoding")]
    pub decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the environment variable or secret that holds the password. Used when `auth_strategy` is `basic`.
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// TLS configuration for the WebSocket source. Use `enabled` for standard `<wss://`> connections, or `with_client_cert` to present a client certificate for mutual TLS.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTls>,
    /// Name of the environment variable or secret that holds the bearer token. Used when `auth_strategy` is `bearer`.
    #[serde(rename = "token_key")]
    pub token_key: Option<String>,
    /// The source type. The value should always be `websocket`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceType,
    /// Name of the environment variable or secret that holds the WebSocket server URI (`<ws://`> or `<wss://`>).
    #[serde(rename = "uri_key")]
    pub uri_key: Option<String>,
    /// Name of the environment variable or secret that holds the username. Used when `auth_strategy` is `basic`.
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineWebsocketSource {
    pub fn new(
        auth_strategy: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceAuthStrategy,
        decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
        id: String,
        type_: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceType,
    ) -> ObservabilityPipelineWebsocketSource {
        ObservabilityPipelineWebsocketSource {
            auth_strategy,
            custom_key: None,
            decoding,
            id,
            password_key: None,
            tls: None,
            token_key: None,
            type_,
            uri_key: None,
            username_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_key(mut self, value: String) -> Self {
        self.custom_key = Some(value);
        self
    }

    pub fn password_key(mut self, value: String) -> Self {
        self.password_key = Some(value);
        self
    }

    pub fn tls(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTls,
    ) -> Self {
        self.tls = Some(value);
        self
    }

    pub fn token_key(mut self, value: String) -> Self {
        self.token_key = Some(value);
        self
    }

    pub fn uri_key(mut self, value: String) -> Self {
        self.uri_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineWebsocketSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineWebsocketSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineWebsocketSourceVisitor {
            type Value = ObservabilityPipelineWebsocketSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_strategy: Option<
                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceAuthStrategy,
                > = None;
                let mut custom_key: Option<String> = None;
                let mut decoding: Option<crate::datadogV2::model::ObservabilityPipelineDecoding> =
                    None;
                let mut id: Option<String> = None;
                let mut password_key: Option<String> = None;
                let mut tls: Option<
                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTls,
                > = None;
                let mut token_key: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceType,
                > = None;
                let mut uri_key: Option<String> = None;
                let mut username_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_strategy" => {
                            auth_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_strategy) = auth_strategy {
                                match _auth_strategy {
                                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceAuthStrategy::UnparsedObject(_auth_strategy) => {
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
                            if let Some(ref _tls) = tls {
                                match _tls {
                                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTls::UnparsedObject(_tls) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "token_key" => {
                            if v.is_null() {
                                continue;
                            }
                            token_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "uri_key" => {
                            if v.is_null() {
                                continue;
                            }
                            uri_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ObservabilityPipelineWebsocketSource {
                    auth_strategy,
                    custom_key,
                    decoding,
                    id,
                    password_key,
                    tls,
                    token_key,
                    type_,
                    uri_key,
                    username_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineWebsocketSourceVisitor)
    }
}
