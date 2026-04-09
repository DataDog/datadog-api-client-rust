// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Compression configuration for the Elasticsearch destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineElasticsearchDestinationCompression {
    /// The compression algorithm applied when sending data to Elasticsearch.
    #[serde(rename = "algorithm")]
    pub algorithm:
        crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm,
    /// The compression level. Only applicable for `gzip`, `zlib`, and `zstd` algorithms.
    #[serde(rename = "level")]
    pub level: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineElasticsearchDestinationCompression {
    pub fn new(
        algorithm: crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm,
    ) -> ObservabilityPipelineElasticsearchDestinationCompression {
        ObservabilityPipelineElasticsearchDestinationCompression {
            algorithm,
            level: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineElasticsearchDestinationCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineElasticsearchDestinationCompressionVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineElasticsearchDestinationCompressionVisitor {
            type Value = ObservabilityPipelineElasticsearchDestinationCompression;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut algorithm: Option<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm> = None;
                let mut level: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "algorithm" => {
                            algorithm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _algorithm) = algorithm {
                                match _algorithm {
                                    crate::datadogV2::model::ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm::UnparsedObject(_algorithm) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "level" => {
                            if v.is_null() {
                                continue;
                            }
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let algorithm = algorithm.ok_or_else(|| M::Error::missing_field("algorithm"))?;

                let content = ObservabilityPipelineElasticsearchDestinationCompression {
                    algorithm,
                    level,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineElasticsearchDestinationCompressionVisitor)
    }
}
