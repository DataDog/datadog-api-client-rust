// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The array-map processor transforms each element of a source array by applying
/// sub-processors in order and collecting the results into a target array.
/// Results can be written to a new array, to the source array (in-place), or to
/// an existing target array. Sub-processors can read from `$sourceElem.<field>`
/// (object element field), bare `$sourceElem` (primitive element), or any parent
/// log attribute path. Sub-processors write to `$targetElem.<field>` (object
/// output field) or bare `$targetElem` (primitive output).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayMapProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// When `false` and `source != target`, the source attribute is removed after
    /// processing. Cannot be `false` when `source == target`.
    #[serde(rename = "preserve_source")]
    pub preserve_source: Option<bool>,
    /// Sub-processors applied to each element. Allowed types: `attribute-remapper`,
    /// `string-builder-processor`, `arithmetic-processor`, `category-processor`.
    #[serde(rename = "processors")]
    pub processors: Vec<crate::datadogV1::model::LogsArrayMapSubProcessor>,
    /// Attribute path of the source array. Elements are read-only via `$sourceElem`
    /// inside sub-processors.
    #[serde(rename = "source")]
    pub source: String,
    /// Attribute path of the output array. Sub-processors write to `$targetElem`
    /// (or `$targetElem.<field>`) to build each output element.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs array-map processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsArrayMapProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayMapProcessor {
    pub fn new(
        processors: Vec<crate::datadogV1::model::LogsArrayMapSubProcessor>,
        source: String,
        target: String,
        type_: crate::datadogV1::model::LogsArrayMapProcessorType,
    ) -> LogsArrayMapProcessor {
        LogsArrayMapProcessor {
            is_enabled: None,
            name: None,
            preserve_source: None,
            processors,
            source,
            target,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn preserve_source(mut self, value: bool) -> Self {
        self.preserve_source = Some(value);
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

impl<'de> Deserialize<'de> for LogsArrayMapProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayMapProcessorVisitor;
        impl<'a> Visitor<'a> for LogsArrayMapProcessorVisitor {
            type Value = LogsArrayMapProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut preserve_source: Option<bool> = None;
                let mut processors: Option<Vec<crate::datadogV1::model::LogsArrayMapSubProcessor>> =
                    None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsArrayMapProcessorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "preserve_source" => {
                            if v.is_null() {
                                continue;
                            }
                            preserve_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processors" => {
                            processors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsArrayMapProcessorType::UnparsedObject(_type_) => {
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
                let processors = processors.ok_or_else(|| M::Error::missing_field("processors"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArrayMapProcessor {
                    is_enabled,
                    name,
                    preserve_source,
                    processors,
                    source,
                    target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayMapProcessorVisitor)
    }
}
