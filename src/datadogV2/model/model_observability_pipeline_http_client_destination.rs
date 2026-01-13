// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `http_client` destination sends data to an HTTP endpoint.
///
/// **Supported pipeline types:** logs, metrics
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineHttpClientDestination {
    /// HTTP authentication strategy.
    #[serde(rename = "auth_strategy")]
    pub auth_strategy:
        Option<crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationAuthStrategy>,
    /// Compression configuration for HTTP requests.
    #[serde(rename = "compression")]
    pub compression:
        Option<crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationCompression>,
    /// Encoding format for log events.
    #[serde(rename = "encoding")]
    pub encoding: crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationEncoding,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the input for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The destination type. The value should always be `http_client`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineHttpClientDestination {
    pub fn new(
        encoding: crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationEncoding,
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationType,
    ) -> ObservabilityPipelineHttpClientDestination {
        ObservabilityPipelineHttpClientDestination {
            auth_strategy: None,
            compression: None,
            encoding,
            id,
            inputs,
            tls: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth_strategy(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationAuthStrategy,
    ) -> Self {
        self.auth_strategy = Some(value);
        self
    }

    pub fn compression(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationCompression,
    ) -> Self {
        self.compression = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineHttpClientDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineHttpClientDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineHttpClientDestinationVisitor {
            type Value = ObservabilityPipelineHttpClientDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_strategy: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationAuthStrategy,
                > = None;
                let mut compression: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationCompression,
                > = None;
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationEncoding,
                > = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationType,
                > = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationAuthStrategy::UnparsedObject(_auth_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "compression" => {
                            if v.is_null() {
                                continue;
                            }
                            compression =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "encoding" => {
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encoding) = encoding {
                                match _encoding {
                                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::ObservabilityPipelineHttpClientDestinationType::UnparsedObject(_type_) => {
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
                let encoding = encoding.ok_or_else(|| M::Error::missing_field("encoding"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineHttpClientDestination {
                    auth_strategy,
                    compression,
                    encoding,
                    id,
                    inputs,
                    tls,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineHttpClientDestinationVisitor)
    }
}
