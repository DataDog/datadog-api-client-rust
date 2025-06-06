// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for keywords used to reinforce sensitive data pattern detection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions {
    /// A list of keywords to match near the sensitive pattern.
    #[serde(rename = "keywords")]
    pub keywords: Vec<String>,
    /// Maximum number of tokens between a keyword and a sensitive value match.
    #[serde(rename = "proximity")]
    pub proximity: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions {
    pub fn new(
        keywords: Vec<String>,
        proximity: i64,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions {
        ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions {
            keywords,
            proximity,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptionsVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptionsVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut keywords: Option<Vec<String>> = None;
                let mut proximity: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "keywords" => {
                            keywords = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "proximity" => {
                            proximity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let keywords = keywords.ok_or_else(|| M::Error::missing_field("keywords"))?;
                let proximity = proximity.ok_or_else(|| M::Error::missing_field("proximity"))?;

                let content = ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptions {
                    keywords,
                    proximity,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineSensitiveDataScannerProcessorKeywordOptionsVisitor,
        )
    }
}
