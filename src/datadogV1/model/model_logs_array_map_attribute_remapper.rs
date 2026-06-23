// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An attribute remapper sub-processor for use inside an array-map processor.
/// Unlike the top-level attribute remapper, `is_enabled`, `source_type`, and
/// `target_type` are not supported.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayMapAttributeRemapper {
    /// Name of the sub-processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Override the target element if already set.
    #[serde(rename = "override_on_conflict")]
    pub override_on_conflict: Option<bool>,
    /// Remove or preserve the remapped source element.
    #[serde(rename = "preserve_source")]
    pub preserve_source: Option<bool>,
    /// Array of source attribute paths.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Target attribute path.
    #[serde(rename = "target")]
    pub target: String,
    /// If the `target_type` of the remapper is `attribute`, try to cast the value to a new specific type.
    /// If the cast is not possible, the original type is kept. `string`, `integer`, or `double` are the possible types.
    /// If the `target_type` is `tag`, this parameter may not be specified.
    #[serde(rename = "target_format")]
    pub target_format: Option<crate::datadogV1::model::TargetFormatType>,
    /// Type of logs attribute remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsAttributeRemapperType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayMapAttributeRemapper {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsAttributeRemapperType,
    ) -> LogsArrayMapAttributeRemapper {
        LogsArrayMapAttributeRemapper {
            name: None,
            override_on_conflict: None,
            preserve_source: None,
            sources,
            target,
            target_format: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn override_on_conflict(mut self, value: bool) -> Self {
        self.override_on_conflict = Some(value);
        self
    }

    pub fn preserve_source(mut self, value: bool) -> Self {
        self.preserve_source = Some(value);
        self
    }

    pub fn target_format(mut self, value: crate::datadogV1::model::TargetFormatType) -> Self {
        self.target_format = Some(value);
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

impl<'de> Deserialize<'de> for LogsArrayMapAttributeRemapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayMapAttributeRemapperVisitor;
        impl<'a> Visitor<'a> for LogsArrayMapAttributeRemapperVisitor {
            type Value = LogsArrayMapAttributeRemapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut override_on_conflict: Option<bool> = None;
                let mut preserve_source: Option<bool> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut target: Option<String> = None;
                let mut target_format: Option<crate::datadogV1::model::TargetFormatType> = None;
                let mut type_: Option<crate::datadogV1::model::LogsAttributeRemapperType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_on_conflict" => {
                            if v.is_null() {
                                continue;
                            }
                            override_on_conflict =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "preserve_source" => {
                            if v.is_null() {
                                continue;
                            }
                            preserve_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_format" => {
                            if v.is_null() {
                                continue;
                            }
                            target_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target_format) = target_format {
                                match _target_format {
                                    crate::datadogV1::model::TargetFormatType::UnparsedObject(
                                        _target_format,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsAttributeRemapperType::UnparsedObject(_type_) => {
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
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArrayMapAttributeRemapper {
                    name,
                    override_on_conflict,
                    preserve_source,
                    sources,
                    target,
                    target_format,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayMapAttributeRemapperVisitor)
    }
}
