// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a custom regex-based pattern for identifying sensitive data in logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern {
    /// Options for defining a custom regex pattern.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternOptions,
    /// Indicates a custom regular expression is used for matching.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern {
    pub fn new(
        options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternOptions,
        type_: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternType,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern {
        ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternVisitor {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut options: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternOptions> = None;
                let mut type_: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternType> = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternType::UnparsedObject(_type_) => {
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

                let content = ObservabilityPipelineSensitiveDataScannerProcessorCustomPattern {
                    options,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSensitiveDataScannerProcessorCustomPatternVisitor)
    }
}
