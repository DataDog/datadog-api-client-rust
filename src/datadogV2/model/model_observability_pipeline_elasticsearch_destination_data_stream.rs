// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration options for writing to Elasticsearch Data Streams instead of a fixed index.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineElasticsearchDestinationDataStream {
    /// The data stream dataset for your logs. This groups logs by their source or application.
    #[serde(rename = "dataset")]
    pub dataset: Option<String>,
    /// The data stream type for your logs. This determines how logs are categorized within the data stream.
    #[serde(rename = "dtype")]
    pub dtype: Option<String>,
    /// The data stream namespace for your logs. This separates logs into different environments or domains.
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineElasticsearchDestinationDataStream {
    pub fn new() -> ObservabilityPipelineElasticsearchDestinationDataStream {
        ObservabilityPipelineElasticsearchDestinationDataStream {
            dataset: None,
            dtype: None,
            namespace: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataset(mut self, value: String) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn dtype(mut self, value: String) -> Self {
        self.dtype = Some(value);
        self
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = Some(value);
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

impl Default for ObservabilityPipelineElasticsearchDestinationDataStream {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineElasticsearchDestinationDataStream {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineElasticsearchDestinationDataStreamVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineElasticsearchDestinationDataStreamVisitor {
            type Value = ObservabilityPipelineElasticsearchDestinationDataStream;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset: Option<String> = None;
                let mut dtype: Option<String> = None;
                let mut namespace: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset" => {
                            if v.is_null() {
                                continue;
                            }
                            dataset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dtype" => {
                            if v.is_null() {
                                continue;
                            }
                            dtype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineElasticsearchDestinationDataStream {
                    dataset,
                    dtype,
                    namespace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineElasticsearchDestinationDataStreamVisitor)
    }
}
