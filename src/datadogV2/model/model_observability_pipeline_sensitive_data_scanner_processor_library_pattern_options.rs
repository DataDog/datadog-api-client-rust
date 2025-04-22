// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions {
    /// Identifier for a predefined pattern from the sensitive data scanner pattern library.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether to augment the pattern with recommended keywords (optional).
    #[serde(rename = "use_recommended_keywords")]
    pub use_recommended_keywords: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions {
    pub fn new(
        id: String,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions {
        ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions {
            id,
            use_recommended_keywords: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn use_recommended_keywords(mut self, value: bool) -> Self {
        self.use_recommended_keywords = Some(value);
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

impl<'de> Deserialize<'de>
    for ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptionsVisitor;
        impl<'a> Visitor<'a>
            for ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptionsVisitor
        {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut use_recommended_keywords: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "use_recommended_keywords" => {
                            if v.is_null() {
                                continue;
                            }
                            use_recommended_keywords =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content =
                    ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions {
                        id,
                        use_recommended_keywords,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptionsVisitor,
        )
    }
}
