// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `prometheus_remote_write` source ingests metrics pushed over the Prometheus Remote Write protocol.
///
/// **Supported pipeline types:** metrics
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelinePrometheusRemoteWriteSource {
    /// Name of the environment variable or secret that holds the listen address for the Prometheus Remote Write endpoint.
    #[serde(rename = "address_key")]
    pub address_key: Option<String>,
    /// HTTP authentication method.
    #[serde(rename = "auth_strategy")]
    pub auth_strategy:
        crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceAuthStrategy,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the environment variable or secret that holds the password (used when `auth_strategy` is `plain`).
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// The HTTP path on which the source listens for incoming Prometheus Remote Write requests.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Configuration for enabling TLS encryption between the pipeline component and external connecting clients.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineMtlsServerTls>,
    /// The source type. The value should always be `prometheus_remote_write`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceType,
    /// Name of the environment variable or secret that holds the username (used when `auth_strategy` is `plain`).
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    /// A list of tokens that are accepted for authenticating incoming requests. When set,
    /// the source rejects any request whose token does not match an enabled entry in this list.
    #[serde(rename = "valid_tokens")]
    pub valid_tokens: Option<
        Vec<crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceValidToken>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelinePrometheusRemoteWriteSource {
    pub fn new(
        auth_strategy: crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceAuthStrategy,
        id: String,
        type_: crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceType,
    ) -> ObservabilityPipelinePrometheusRemoteWriteSource {
        ObservabilityPipelinePrometheusRemoteWriteSource {
            address_key: None,
            auth_strategy,
            id,
            password_key: None,
            path: None,
            tls: None,
            type_,
            username_key: None,
            valid_tokens: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn address_key(mut self, value: String) -> Self {
        self.address_key = Some(value);
        self
    }

    pub fn password_key(mut self, value: String) -> Self {
        self.password_key = Some(value);
        self
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    pub fn tls(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineMtlsServerTls,
    ) -> Self {
        self.tls = Some(value);
        self
    }

    pub fn username_key(mut self, value: String) -> Self {
        self.username_key = Some(value);
        self
    }

    pub fn valid_tokens(
        mut self,
        value: Vec<
            crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceValidToken,
        >,
    ) -> Self {
        self.valid_tokens = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelinePrometheusRemoteWriteSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelinePrometheusRemoteWriteSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelinePrometheusRemoteWriteSourceVisitor {
            type Value = ObservabilityPipelinePrometheusRemoteWriteSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut address_key: Option<String> = None;
                let mut auth_strategy: Option<crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceAuthStrategy> = None;
                let mut id: Option<String> = None;
                let mut password_key: Option<String> = None;
                let mut path: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineMtlsServerTls> =
                    None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceType,
                > = None;
                let mut username_key: Option<String> = None;
                let mut valid_tokens: Option<Vec<crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceValidToken>> = None;
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
                                    crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceAuthStrategy::UnparsedObject(_auth_strategy) => {
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
                        "path" => {
                            if v.is_null() {
                                continue;
                            }
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::ObservabilityPipelinePrometheusRemoteWriteSourceType::UnparsedObject(_type_) => {
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
                        "valid_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            valid_tokens =
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelinePrometheusRemoteWriteSource {
                    address_key,
                    auth_strategy,
                    id,
                    password_key,
                    path,
                    tls,
                    type_,
                    username_key,
                    valid_tokens,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelinePrometheusRemoteWriteSourceVisitor)
    }
}
