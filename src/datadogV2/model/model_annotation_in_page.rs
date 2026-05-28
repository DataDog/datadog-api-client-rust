// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A flat annotation object as it appears within a page annotations response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotationInPage {
    /// Identifier of the user who created the annotation.
    #[serde(rename = "author_id")]
    pub author_id: String,
    /// Color used to render the annotation in the UI.
    #[serde(rename = "color")]
    pub color: crate::datadogV2::model::AnnotationColor,
    /// Creation time of the annotation in milliseconds since the Unix epoch.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// User-defined text attached to the annotation.
    #[serde(rename = "description")]
    pub description: String,
    /// End time of the annotation in milliseconds since the Unix epoch. Null for `pointInTime` annotations.
    #[serialize_always]
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    /// Unique identifier of the annotation.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Last modification time of the annotation in milliseconds since the Unix epoch.
    #[serde(rename = "modified_at")]
    pub modified_at: i64,
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

impl AnnotationInPage {
    pub fn new(
        author_id: String,
        color: crate::datadogV2::model::AnnotationColor,
        created_at: i64,
        description: String,
        end_time: Option<i64>,
        id: uuid::Uuid,
        modified_at: i64,
        page_id: String,
        start_time: i64,
        type_: crate::datadogV2::model::AnnotationKind,
    ) -> AnnotationInPage {
        AnnotationInPage {
            author_id,
            color,
            created_at,
            description,
            end_time,
            id,
            modified_at,
            page_id,
            start_time,
            type_,
            widget_ids: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for AnnotationInPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnnotationInPageVisitor;
        impl<'a> Visitor<'a> for AnnotationInPageVisitor {
            type Value = AnnotationInPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author_id: Option<String> = None;
                let mut color: Option<crate::datadogV2::model::AnnotationColor> = None;
                let mut created_at: Option<i64> = None;
                let mut description: Option<String> = None;
                let mut end_time: Option<Option<i64>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut modified_at: Option<i64> = None;
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
                        "author_id" => {
                            author_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_time" => {
                            end_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let author_id = author_id.ok_or_else(|| M::Error::missing_field("author_id"))?;
                let color = color.ok_or_else(|| M::Error::missing_field("color"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let end_time = end_time.ok_or_else(|| M::Error::missing_field("end_time"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let page_id = page_id.ok_or_else(|| M::Error::missing_field("page_id"))?;
                let start_time = start_time.ok_or_else(|| M::Error::missing_field("start_time"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AnnotationInPage {
                    author_id,
                    color,
                    created_at,
                    description,
                    end_time,
                    id,
                    modified_at,
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

        deserializer.deserialize_any(AnnotationInPageVisitor)
    }
}
