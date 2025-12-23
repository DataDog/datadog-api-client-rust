// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `elasticsearch` destination writes logs to an Elasticsearch cluster.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineElasticsearchDestination {
    /// The Elasticsearch API version to use. Set to `auto` to auto-detect.
    #[serde(rename = "api_version")]
    pub api_version:
        Option<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationApiVersion>,
    /// The index to write logs to in Elasticsearch.
    #[serde(rename = "bulk_index")]
    pub bulk_index: Option<String>,
    /// Configuration options for writing to Elasticsearch Data Streams instead of a fixed index.
    #[serde(rename = "data_stream")]
    pub data_stream:
        Option<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationDataStream>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The destination type. The value should always be `elasticsearch`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineElasticsearchDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationType,
    ) -> ObservabilityPipelineElasticsearchDestination {
        ObservabilityPipelineElasticsearchDestination {
            api_version: None,
            bulk_index: None,
            data_stream: None,
            id,
            inputs,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_version(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationApiVersion,
    ) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn bulk_index(mut self, value: String) -> Self {
        self.bulk_index = Some(value);
        self
    }

    pub fn data_stream(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationDataStream,
    ) -> Self {
        self.data_stream = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineElasticsearchDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineElasticsearchDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineElasticsearchDestinationVisitor {
            type Value = ObservabilityPipelineElasticsearchDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_version: Option<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationApiVersion> = None;
                let mut bulk_index: Option<String> = None;
                let mut data_stream: Option<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationDataStream> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_version" => {
                            if v.is_null() {
                                continue;
                            }
                            api_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _api_version) = api_version {
                                match _api_version {
                                    crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationApiVersion::UnparsedObject(_api_version) => {
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
                                    crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationType::UnparsedObject(_type_) => {
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

                let content = ObservabilityPipelineElasticsearchDestination {
                    api_version,
                    bulk_index,
                    data_stream,
                    id,
                    inputs,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineElasticsearchDestinationVisitor)
    }
}
