// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of annotations used in the workflow. These are like sticky notes for your workflow!
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Annotation {
    /// The definition of `AnnotationDisplay` object.
    #[serde(rename = "display")]
    pub display: crate::datadogV2::model::AnnotationDisplay,
    /// The `Annotation` `id`.
    #[serde(rename = "id")]
    pub id: String,
    /// The definition of `AnnotationMarkdownTextAnnotation` object.
    #[serde(rename = "markdownTextAnnotation")]
    pub markdown_text_annotation: crate::datadogV2::model::AnnotationMarkdownTextAnnotation,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Annotation {
    pub fn new(
        display: crate::datadogV2::model::AnnotationDisplay,
        id: String,
        markdown_text_annotation: crate::datadogV2::model::AnnotationMarkdownTextAnnotation,
    ) -> Annotation {
        Annotation {
            display,
            id,
            markdown_text_annotation,
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

impl<'de> Deserialize<'de> for Annotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnnotationVisitor;
        impl<'a> Visitor<'a> for AnnotationVisitor {
            type Value = Annotation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display: Option<crate::datadogV2::model::AnnotationDisplay> = None;
                let mut id: Option<String> = None;
                let mut markdown_text_annotation: Option<
                    crate::datadogV2::model::AnnotationMarkdownTextAnnotation,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display" => {
                            display = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "markdownTextAnnotation" => {
                            markdown_text_annotation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let display = display.ok_or_else(|| M::Error::missing_field("display"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let markdown_text_annotation = markdown_text_annotation
                    .ok_or_else(|| M::Error::missing_field("markdown_text_annotation"))?;

                let content = Annotation {
                    display,
                    id,
                    markdown_text_annotation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnnotationVisitor)
    }
}
