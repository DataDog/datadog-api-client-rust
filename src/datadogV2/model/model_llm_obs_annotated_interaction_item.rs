// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An interaction with its associated annotations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotatedInteractionItem {
    /// List of annotations for this interaction.
    #[serde(rename = "annotations")]
    pub annotations: Vec<crate::datadogV2::model::LLMObsAnnotationItem>,
    /// Identifier of the content (trace ID or session ID) for this interaction.
    #[serde(rename = "content_id")]
    pub content_id: String,
    /// Unique identifier of the interaction.
    #[serde(rename = "id")]
    pub id: String,
    /// Type of interaction in an annotation queue.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsInteractionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotatedInteractionItem {
    pub fn new(
        annotations: Vec<crate::datadogV2::model::LLMObsAnnotationItem>,
        content_id: String,
        id: String,
        type_: crate::datadogV2::model::LLMObsInteractionType,
    ) -> LLMObsAnnotatedInteractionItem {
        LLMObsAnnotatedInteractionItem {
            annotations,
            content_id,
            id,
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

impl<'de> Deserialize<'de> for LLMObsAnnotatedInteractionItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotatedInteractionItemVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotatedInteractionItemVisitor {
            type Value = LLMObsAnnotatedInteractionItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotations: Option<Vec<crate::datadogV2::model::LLMObsAnnotationItem>> =
                    None;
                let mut content_id: Option<String> = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LLMObsInteractionType> = None;
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
                        "content_id" => {
                            content_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LLMObsInteractionType::UnparsedObject(_type_) => {
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
                let annotations =
                    annotations.ok_or_else(|| M::Error::missing_field("annotations"))?;
                let content_id = content_id.ok_or_else(|| M::Error::missing_field("content_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsAnnotatedInteractionItem {
                    annotations,
                    content_id,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotatedInteractionItemVisitor)
    }
}
