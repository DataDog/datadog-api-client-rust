// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `amazon_opensearch` destination writes logs to Amazon OpenSearch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAmazonOpenSearchDestination {
    /// Authentication settings for the Amazon OpenSearch destination.
    /// The `strategy` field determines whether basic or AWS-based authentication is used.
    #[serde(rename = "auth")]
    pub auth: crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuth,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// The index to write logs to.
    #[serde(rename = "bulk_index")]
    pub bulk_index: Option<String>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The destination type. The value should always be `amazon_opensearch`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAmazonOpenSearchDestination {
    pub fn new(
        auth: crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuth,
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationType,
    ) -> ObservabilityPipelineAmazonOpenSearchDestination {
        ObservabilityPipelineAmazonOpenSearchDestination {
            auth,
            buffer: None,
            bulk_index: None,
            id,
            inputs,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonOpenSearchDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAmazonOpenSearchDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAmazonOpenSearchDestinationVisitor {
            type Value = ObservabilityPipelineAmazonOpenSearchDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuth,
                > = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut bulk_index: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
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
                                    crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationType::UnparsedObject(_type_) => {
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
                let auth = auth.ok_or_else(|| M::Error::missing_field("auth"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineAmazonOpenSearchDestination {
                    auth,
                    buffer,
                    bulk_index,
                    id,
                    inputs,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAmazonOpenSearchDestinationVisitor)
    }
}
