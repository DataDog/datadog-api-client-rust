// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A category sub-processor for use inside an array-map processor.
/// Unlike the top-level category processor, `is_enabled` is not supported.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayMapCategorySubProcessor {
    /// Array of filters to match against a log and the corresponding value to assign.
    #[serde(rename = "categories")]
    pub categories: Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
    /// Name of the sub-processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Target attribute path for the category value.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs category processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsCategoryProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayMapCategorySubProcessor {
    pub fn new(
        categories: Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
        target: String,
        type_: crate::datadogV1::model::LogsCategoryProcessorType,
    ) -> LogsArrayMapCategorySubProcessor {
        LogsArrayMapCategorySubProcessor {
            categories,
            name: None,
            target,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl<'de> Deserialize<'de> for LogsArrayMapCategorySubProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayMapCategorySubProcessorVisitor;
        impl<'a> Visitor<'a> for LogsArrayMapCategorySubProcessorVisitor {
            type Value = LogsArrayMapCategorySubProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut categories: Option<
                    Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
                > = None;
                let mut name: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsCategoryProcessorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "categories" => {
                            categories = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsCategoryProcessorType::UnparsedObject(_type_) => {
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
                let categories = categories.ok_or_else(|| M::Error::missing_field("categories"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArrayMapCategorySubProcessor {
                    categories,
                    name,
                    target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayMapCategorySubProcessorVisitor)
    }
}
