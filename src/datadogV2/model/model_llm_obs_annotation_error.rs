// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A partial error for a single annotation that could not be processed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationError {
    /// ID of the annotation that failed, if applicable.
    #[serde(rename = "annotation_id")]
    pub annotation_id: Option<String>,
    /// Error message.
    #[serde(rename = "error")]
    pub error: String,
    /// ID of the interaction that failed.
    #[serde(rename = "interaction_id")]
    pub interaction_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationError {
    pub fn new(error: String, interaction_id: String) -> LLMObsAnnotationError {
        LLMObsAnnotationError {
            annotation_id: None,
            error,
            interaction_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn annotation_id(mut self, value: String) -> Self {
        self.annotation_id = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotationError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationErrorVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationErrorVisitor {
            type Value = LLMObsAnnotationError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotation_id: Option<String> = None;
                let mut error: Option<String> = None;
                let mut interaction_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotation_id" => {
                            if v.is_null() {
                                continue;
                            }
                            annotation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interaction_id" => {
                            interaction_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error = error.ok_or_else(|| M::Error::missing_field("error"))?;
                let interaction_id =
                    interaction_id.ok_or_else(|| M::Error::missing_field("interaction_id"))?;

                let content = LLMObsAnnotationError {
                    annotation_id,
                    error,
                    interaction_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationErrorVisitor)
    }
}
