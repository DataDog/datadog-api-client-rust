// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the annotations on a page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PageAnnotationsAttributes {
    /// Map of annotation UUID to annotation object, keyed by annotation ID.
    #[serde(rename = "annotations")]
    pub annotations: std::collections::BTreeMap<String, crate::datadogV2::model::AnnotationInPage>,
    /// List of annotation IDs that apply to the entire page rather than a specific widget.
    #[serde(rename = "global_annotations")]
    pub global_annotations: Vec<uuid::Uuid>,
    /// Map from widget ID to the list of annotation IDs displayed on that widget.
    #[serde(rename = "widget_mapping")]
    pub widget_mapping: std::collections::BTreeMap<String, Vec<uuid::Uuid>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PageAnnotationsAttributes {
    pub fn new(
        annotations: std::collections::BTreeMap<String, crate::datadogV2::model::AnnotationInPage>,
        global_annotations: Vec<uuid::Uuid>,
        widget_mapping: std::collections::BTreeMap<String, Vec<uuid::Uuid>>,
    ) -> PageAnnotationsAttributes {
        PageAnnotationsAttributes {
            annotations,
            global_annotations,
            widget_mapping,
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

impl<'de> Deserialize<'de> for PageAnnotationsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PageAnnotationsAttributesVisitor;
        impl<'a> Visitor<'a> for PageAnnotationsAttributesVisitor {
            type Value = PageAnnotationsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotations: Option<
                    std::collections::BTreeMap<String, crate::datadogV2::model::AnnotationInPage>,
                > = None;
                let mut global_annotations: Option<Vec<uuid::Uuid>> = None;
                let mut widget_mapping: Option<
                    std::collections::BTreeMap<String, Vec<uuid::Uuid>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotations" => {
                            annotations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_annotations" => {
                            global_annotations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widget_mapping" => {
                            widget_mapping =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotations =
                    annotations.ok_or_else(|| M::Error::missing_field("annotations"))?;
                let global_annotations = global_annotations
                    .ok_or_else(|| M::Error::missing_field("global_annotations"))?;
                let widget_mapping =
                    widget_mapping.ok_or_else(|| M::Error::missing_field("widget_mapping"))?;

                let content = PageAnnotationsAttributes {
                    annotations,
                    global_annotations,
                    widget_mapping,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PageAnnotationsAttributesVisitor)
    }
}
