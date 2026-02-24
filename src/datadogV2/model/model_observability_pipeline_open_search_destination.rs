// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `opensearch` destination writes logs to an OpenSearch cluster.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOpenSearchDestination {
    /// Authentication settings for the Elasticsearch destination.
    /// When `strategy` is `basic`, use `username_key` and `password_key` to reference credentials stored in environment variables or secrets.
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationAuth>,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// The index to write logs to.
    #[serde(rename = "bulk_index")]
    pub bulk_index: Option<String>,
    /// Configuration options for writing to OpenSearch Data Streams instead of a fixed index.
    #[serde(rename = "data_stream")]
    pub data_stream:
        Option<crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationDataStream>,
    /// Name of the environment variable or secret that holds the OpenSearch endpoint URL.
    #[serde(rename = "endpoint_url_key")]
    pub endpoint_url_key: Option<String>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The destination type. The value should always be `opensearch`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOpenSearchDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationType,
    ) -> ObservabilityPipelineOpenSearchDestination {
        ObservabilityPipelineOpenSearchDestination {
            auth: None,
            buffer: None,
            bulk_index: None,
            data_stream: None,
            endpoint_url_key: None,
            id,
            inputs,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationAuth,
    ) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn bulk_index(mut self, value: String) -> Self {
        self.bulk_index = Some(value);
        self
    }

    pub fn data_stream(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationDataStream,
    ) -> Self {
        self.data_stream = Some(value);
        self
    }

    pub fn endpoint_url_key(mut self, value: String) -> Self {
        self.endpoint_url_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineOpenSearchDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOpenSearchDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOpenSearchDestinationVisitor {
            type Value = ObservabilityPipelineOpenSearchDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationAuth,
                > = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut bulk_index: Option<String> = None;
                let mut data_stream: Option<
                    crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationDataStream,
                > = None;
                let mut endpoint_url_key: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "bulk_index" => {
                            if v.is_null() {
                                continue;
                            }
                            bulk_index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_stream" => {
                            if v.is_null() {
                                continue;
                            }
                            data_stream =
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineOpenSearchDestinationType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineOpenSearchDestination {
                    auth,
                    buffer,
                    bulk_index,
                    data_stream,
                    endpoint_url_key,
                    id,
                    inputs,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOpenSearchDestinationVisitor)
    }
}
