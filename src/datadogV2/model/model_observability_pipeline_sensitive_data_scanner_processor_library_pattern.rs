// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Specifies a pattern from Datadog’s sensitive data detection library to match known sensitive data types.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern {
    /// Options for selecting a predefined library pattern and enabling keyword support.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions,
    /// Indicates that a predefined library pattern is used.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern {
    pub fn new(
        options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions,
        type_: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternType,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern {
        ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern {
            options,
            type_,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut options: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternOptions> = None;
                let mut type_: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternType::UnparsedObject(_type_) => {
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
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSensitiveDataScannerProcessorLibraryPattern {
                    options,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineSensitiveDataScannerProcessorLibraryPatternVisitor,
        )
    }
}
