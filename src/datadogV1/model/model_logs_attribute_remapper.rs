// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The remapper processor remaps any source attribute(s) or tag to another target attribute or tag.
/// Constraints on the tag/attribute name are explained in the [Tag Best Practice documentation](<https://docs.datadoghq.com/logs/guide/log-parsing-best-practice>).
/// Some additional constraints are applied as `:` or `,` are not allowed in the target tag/attribute name.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsAttributeRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Override or not the target element if already set,
    #[serde(rename = "override_on_conflict")]
    pub override_on_conflict: Option<bool>,
    /// Remove or preserve the remapped source element.
    #[serde(rename = "preserve_source")]
    pub preserve_source: Option<bool>,
    /// Defines if the sources are from log `attribute` or `tag`.
    #[serde(rename = "source_type")]
    pub source_type: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Final attribute or tag name to remap the sources to.
    #[serde(rename = "target")]
    pub target: String,
    /// If the `target_type` of the remapper is `attribute`, try to cast the value to a new specific type.
    /// If the cast is not possible, the original type is kept. `string`, `integer`, or `double` are the possible types.
    /// If the `target_type` is `tag`, this parameter may not be specified.
    #[serde(rename = "target_format")]
    pub target_format: Option<crate::datadogV1::model::TargetFormatType>,
    /// Defines if the final attribute or tag name is from log `attribute` or `tag`.
    #[serde(rename = "target_type")]
    pub target_type: Option<String>,
    /// Type of logs attribute remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsAttributeRemapperType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsAttributeRemapper {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsAttributeRemapperType,
    ) -> LogsAttributeRemapper {
        LogsAttributeRemapper {
            is_enabled: None,
            name: None,
            override_on_conflict: None,
            preserve_source: None,
            source_type: None,
            sources,
            target,
            target_format: None,
            target_type: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn override_on_conflict(&mut self, value: bool) -> &mut Self {
        self.override_on_conflict = Some(value);
        self
    }

    pub fn preserve_source(&mut self, value: bool) -> &mut Self {
        self.preserve_source = Some(value);
        self
    }

    pub fn source_type(&mut self, value: String) -> &mut Self {
        self.source_type = Some(value);
        self
    }

    pub fn target_format(&mut self, value: crate::datadogV1::model::TargetFormatType) -> &mut Self {
        self.target_format = Some(value);
        self
    }

    pub fn target_type(&mut self, value: String) -> &mut Self {
        self.target_type = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsAttributeRemapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsAttributeRemapperVisitor;
        impl<'a> Visitor<'a> for LogsAttributeRemapperVisitor {
            type Value = LogsAttributeRemapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut override_on_conflict: Option<bool> = None;
                let mut preserve_source: Option<bool> = None;
                let mut source_type: Option<String> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut target: Option<String> = None;
                let mut target_format: Option<crate::datadogV1::model::TargetFormatType> = None;
                let mut target_type: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsAttributeRemapperType> = None;
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
                        "source_type" => {
                            if v.is_null() {
                                continue;
                            }
                            source_type =
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
                        "target_type" => {
                            if v.is_null() {
                                continue;
                            }
                            target_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsAttributeRemapper {
                    is_enabled,
                    name,
                    override_on_conflict,
                    preserve_source,
                    source_type,
                    sources,
                    target,
                    target_format,
                    target_type,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsAttributeRemapperVisitor)
    }
}
