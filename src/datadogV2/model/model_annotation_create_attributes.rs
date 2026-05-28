// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating an annotation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotationCreateAttributes {
    /// Color used to render the annotation in the UI.
    #[serde(rename = "color")]
    pub color: crate::datadogV2::model::AnnotationColor,
    /// User-defined text attached to the annotation.
    #[serde(rename = "description")]
    pub description: String,
    /// End time of the annotation in milliseconds since the Unix epoch. Required for `timeRegion` annotations; omit or set to null for `pointInTime` annotations.
    #[serde(
        rename = "end_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub end_time: Option<Option<i64>>,
    /// ID of the page the annotation belongs to, prefixed with the page type and joined by a colon
    /// (for example, `dashboard:abc-def-xyz` or `notebook:1234567890`).
    #[serde(rename = "page_id")]
    pub page_id: String,
    /// Start time of the annotation in milliseconds since the Unix epoch.
    #[serde(rename = "start_time")]
    pub start_time: i64,
    /// Kind of annotation. `pointInTime` annotations mark a single moment in time,
    /// while `timeRegion` annotations span a window of time and require an `end_time`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AnnotationKind,
    /// IDs of widgets the annotation is associated with. When empty or omitted, the annotation applies to the whole page.
    #[serde(rename = "widget_ids")]
    pub widget_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnnotationCreateAttributes {
    pub fn new(
        color: crate::datadogV2::model::AnnotationColor,
        description: String,
        page_id: String,
        start_time: i64,
        type_: crate::datadogV2::model::AnnotationKind,
    ) -> AnnotationCreateAttributes {
        AnnotationCreateAttributes {
            color,
            description,
            end_time: None,
            page_id,
            start_time,
            type_,
            widget_ids: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end_time(mut self, value: Option<i64>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn widget_ids(mut self, value: Vec<String>) -> Self {
        self.widget_ids = Some(value);
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

impl<'de> Deserialize<'de> for AnnotationCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnnotationCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AnnotationCreateAttributesVisitor {
            type Value = AnnotationCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color: Option<crate::datadogV2::model::AnnotationColor> = None;
                let mut description: Option<String> = None;
                let mut end_time: Option<Option<i64>> = None;
                let mut page_id: Option<String> = None;
                let mut start_time: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::AnnotationKind> = None;
                let mut widget_ids: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color" => {
                            color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color) = color {
                                match _color {
                                    crate::datadogV2::model::AnnotationColor::UnparsedObject(
                                        _color,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_time" => {
                            end_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_id" => {
                            page_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_time" => {
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AnnotationKind::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "widget_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            widget_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let color = color.ok_or_else(|| M::Error::missing_field("color"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let page_id = page_id.ok_or_else(|| M::Error::missing_field("page_id"))?;
                let start_time = start_time.ok_or_else(|| M::Error::missing_field("start_time"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AnnotationCreateAttributes {
                    color,
                    description,
                    end_time,
                    page_id,
                    start_time,
                    type_,
                    widget_ids,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnnotationCreateAttributesVisitor)
    }
}
