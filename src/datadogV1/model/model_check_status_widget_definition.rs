// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Check status shows the current status or number of results for any check performed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CheckStatusWidgetDefinition {
    /// Name of the check to use in the widget.
    #[serde(rename = "check")]
    pub check: String,
    /// Group reporting a single check.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// The kind of grouping to use.
    #[serde(rename = "grouping")]
    pub grouping: crate::datadogV1::model::WidgetGrouping,
    /// List of tags used to filter the groups reporting a cluster check.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the check status widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::CheckStatusWidgetDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CheckStatusWidgetDefinition {
    pub fn new(
        check: String,
        grouping: crate::datadogV1::model::WidgetGrouping,
        type_: crate::datadogV1::model::CheckStatusWidgetDefinitionType,
    ) -> CheckStatusWidgetDefinition {
        CheckStatusWidgetDefinition {
            check,
            group: None,
            group_by: None,
            grouping,
            tags: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn group(mut self, value: String) -> Self {
        self.group = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CheckStatusWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckStatusWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for CheckStatusWidgetDefinitionVisitor {
            type Value = CheckStatusWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut check: Option<String> = None;
                let mut group: Option<String> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut grouping: Option<crate::datadogV1::model::WidgetGrouping> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::CheckStatusWidgetDefinitionType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "check" => {
                            check = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grouping" => {
                            grouping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _grouping) = grouping {
                                match _grouping {
                                    crate::datadogV1::model::WidgetGrouping::UnparsedObject(
                                        _grouping,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _title_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::CheckStatusWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let check = check.ok_or_else(|| M::Error::missing_field("check"))?;
                let grouping = grouping.ok_or_else(|| M::Error::missing_field("grouping"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CheckStatusWidgetDefinition {
                    check,
                    group,
                    group_by,
                    grouping,
                    tags,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CheckStatusWidgetDefinitionVisitor)
    }
}
