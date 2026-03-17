// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Gzip compression.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAmazonS3GenericCompressionGzip {
    /// The compression type. Always `gzip`.
    #[serde(rename = "algorithm")]
    pub algorithm: crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompressionGzipType,
    /// Gzip compression level.
    #[serde(rename = "level")]
    pub level: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAmazonS3GenericCompressionGzip {
    pub fn new(
        algorithm: crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompressionGzipType,
        level: i64,
    ) -> ObservabilityPipelineAmazonS3GenericCompressionGzip {
        ObservabilityPipelineAmazonS3GenericCompressionGzip {
            algorithm,
            level,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonS3GenericCompressionGzip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAmazonS3GenericCompressionGzipVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAmazonS3GenericCompressionGzipVisitor {
            type Value = ObservabilityPipelineAmazonS3GenericCompressionGzip;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut algorithm: Option<crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompressionGzipType> = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompressionGzipType::UnparsedObject(_algorithm) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "level" => {
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
                let level = level.ok_or_else(|| M::Error::missing_field("level"))?;

                let content = ObservabilityPipelineAmazonS3GenericCompressionGzip {
                    algorithm,
                    level,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAmazonS3GenericCompressionGzipVisitor)
    }
}
