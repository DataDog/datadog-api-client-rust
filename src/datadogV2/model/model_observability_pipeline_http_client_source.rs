// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `http_client` source scrapes logs from HTTP endpoints at regular intervals.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineHttpClientSource {
    /// Optional authentication strategy for HTTP requests.
    #[serde(rename = "auth_strategy")]
    pub auth_strategy:
        Option<crate::datadogV2::model::ObservabilityPipelineHttpClientSourceAuthStrategy>,
    /// Name of the environment variable or secret that holds a custom header value (used with custom auth strategies).
    #[serde(rename = "custom_key")]
    pub custom_key: Option<String>,
    /// The decoding format used to interpret incoming logs.
    #[serde(rename = "decoding")]
    pub decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
    /// Name of the environment variable or secret that holds the HTTP endpoint URL to scrape.
    #[serde(rename = "endpoint_url_key")]
    pub endpoint_url_key: Option<String>,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the environment variable or secret that holds the password (used when `auth_strategy` is `basic`).
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// The interval (in seconds) between HTTP scrape requests.
    #[serde(rename = "scrape_interval_secs")]
    pub scrape_interval_secs: Option<i64>,
    /// The timeout (in seconds) for each scrape request.
    #[serde(rename = "scrape_timeout_secs")]
    pub scrape_timeout_secs: Option<i64>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// Name of the environment variable or secret that holds the bearer token (used when `auth_strategy` is `bearer`).
    #[serde(rename = "token_key")]
    pub token_key: Option<String>,
    /// The source type. The value should always be `http_client`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineHttpClientSourceType,
    /// Name of the environment variable or secret that holds the username (used when `auth_strategy` is `basic`).
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineHttpClientSource {
    pub fn new(
        decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
        id: String,
        type_: crate::datadogV2::model::ObservabilityPipelineHttpClientSourceType,
    ) -> ObservabilityPipelineHttpClientSource {
        ObservabilityPipelineHttpClientSource {
            auth_strategy: None,
            custom_key: None,
            decoding,
            endpoint_url_key: None,
            id,
            password_key: None,
            scrape_interval_secs: None,
            scrape_timeout_secs: None,
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
        value: crate::datadogV2::model::ObservabilityPipelineHttpClientSourceAuthStrategy,
    ) -> Self {
        self.auth_strategy = Some(value);
        self
    }

    pub fn custom_key(mut self, value: String) -> Self {
        self.custom_key = Some(value);
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

    pub fn scrape_interval_secs(mut self, value: i64) -> Self {
        self.scrape_interval_secs = Some(value);
        self
    }

    pub fn scrape_timeout_secs(mut self, value: i64) -> Self {
        self.scrape_timeout_secs = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineHttpClientSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineHttpClientSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineHttpClientSourceVisitor {
            type Value = ObservabilityPipelineHttpClientSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_strategy: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpClientSourceAuthStrategy,
                > = None;
                let mut custom_key: Option<String> = None;
                let mut decoding: Option<crate::datadogV2::model::ObservabilityPipelineDecoding> =
                    None;
                let mut endpoint_url_key: Option<String> = None;
                let mut id: Option<String> = None;
                let mut password_key: Option<String> = None;
                let mut scrape_interval_secs: Option<i64> = None;
                let mut scrape_timeout_secs: Option<i64> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut token_key: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpClientSourceType,
                > = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineHttpClientSourceAuthStrategy::UnparsedObject(_auth_strategy) => {
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
                        "password_key" => {
                            if v.is_null() {
                                continue;
                            }
                            password_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scrape_interval_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            scrape_interval_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scrape_timeout_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            scrape_timeout_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::ObservabilityPipelineHttpClientSourceType::UnparsedObject(_type_) => {
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
                let decoding = decoding.ok_or_else(|| M::Error::missing_field("decoding"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineHttpClientSource {
                    auth_strategy,
                    custom_key,
                    decoding,
                    endpoint_url_key,
                    id,
                    password_key,
                    scrape_interval_secs,
                    scrape_timeout_secs,
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

        deserializer.deserialize_any(ObservabilityPipelineHttpClientSourceVisitor)
    }
}
