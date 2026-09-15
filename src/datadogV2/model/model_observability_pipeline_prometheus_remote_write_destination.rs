// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `prometheus_remote_write` destination forwards metrics to an endpoint that supports the Prometheus Remote Write protocol.
///
/// **Supported pipeline types:** metrics
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelinePrometheusRemoteWriteDestination {
    /// The authentication strategy to use for outgoing Prometheus Remote Write requests.
    #[serde(rename = "auth_strategy")]
    pub auth_strategy: Option<
        crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationAuthStrategy,
    >,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// The default namespace to add as a prefix to metric names that do not already have one.
    #[serde(rename = "default_namespace")]
    pub default_namespace: Option<String>,
    /// Name of the environment variable or secret that holds the Prometheus Remote Write endpoint URL.
    /// Defaults to `DESTINATION_PROMETHEUS_REMOTE_WRITE_ENDPOINT_URL` (prefixed with `DD_OP_` at runtime).
    #[serde(rename = "endpoint_url_key")]
    pub endpoint_url_key: Option<String>,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Name of the environment variable or secret that holds the password (used when `auth_strategy` is `basic`).
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// The tenant ID to include with outgoing requests. Used by multi-tenant Prometheus Remote Write receivers.
    #[serde(rename = "tenant_id")]
    pub tenant_id: Option<String>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineClientTls>,
    /// Name of the environment variable or secret that holds the bearer token (used when `auth_strategy` is `bearer`).
    #[serde(rename = "token_key")]
    pub token_key: Option<String>,
    /// The destination type. The value should always be `prometheus_remote_write`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationType,
    /// Name of the environment variable or secret that holds the username (used when `auth_strategy` is `basic`).
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelinePrometheusRemoteWriteDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationType,
    ) -> ObservabilityPipelinePrometheusRemoteWriteDestination {
        ObservabilityPipelinePrometheusRemoteWriteDestination {
            auth_strategy: None,
            buffer: None,
            default_namespace: None,
            endpoint_url_key: None,
            id,
            inputs,
            password_key: None,
            tenant_id: None,
            tls: None,
            token_key: None,
            type_,
            username_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth_strategy(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationAuthStrategy,
    ) -> Self {
        self.auth_strategy = Some(value);
        self
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn default_namespace(mut self, value: String) -> Self {
        self.default_namespace = Some(value);
        self
    }

    pub fn endpoint_url_key(mut self, value: String) -> Self {
        self.endpoint_url_key = Some(value);
        self
    }

    pub fn password_key(mut self, value: String) -> Self {
        self.password_key = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: String) -> Self {
        self.tenant_id = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineClientTls) -> Self {
        self.tls = Some(value);
        self
    }

    pub fn token_key(mut self, value: String) -> Self {
        self.token_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelinePrometheusRemoteWriteDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelinePrometheusRemoteWriteDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelinePrometheusRemoteWriteDestinationVisitor {
            type Value = ObservabilityPipelinePrometheusRemoteWriteDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_strategy: Option<crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationAuthStrategy> = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut default_namespace: Option<String> = None;
                let mut endpoint_url_key: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut password_key: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineClientTls> = None;
                let mut token_key: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationType> = None;
                let mut username_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_strategy" => {
                            if v.is_null() {
                                continue;
                            }
                            auth_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_strategy) = auth_strategy {
                                match _auth_strategy {
                                    crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationAuthStrategy::UnparsedObject(_auth_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "buffer" => {
                            if v.is_null() {
                                continue;
                            }
                            buffer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _buffer) = buffer {
                                match _buffer {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptions::UnparsedObject(_buffer) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "default_namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            default_namespace =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "endpoint_url_key" => {
                            if v.is_null() {
                                continue;
                            }
                            endpoint_url_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "password_key" => {
                            if v.is_null() {
                                continue;
                            }
                            password_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteDestinationType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelinePrometheusRemoteWriteDestination {
                    auth_strategy,
                    buffer,
                    default_namespace,
                    endpoint_url_key,
                    id,
                    inputs,
                    password_key,
                    tenant_id,
                    tls,
                    token_key,
                    type_,
                    username_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelinePrometheusRemoteWriteDestinationVisitor)
    }
}
