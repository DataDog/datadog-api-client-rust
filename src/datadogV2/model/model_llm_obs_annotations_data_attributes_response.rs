// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the annotations response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationsDataAttributesResponse {
    /// Successfully created or updated annotations.
    #[serde(rename = "annotations")]
    pub annotations: Vec<crate::datadogV2::model::LLMObsAnnotationItemResponse>,
    /// Partial errors for annotations that could not be processed.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV2::model::LLMObsAnnotationError>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationsDataAttributesResponse {
    pub fn new(
        annotations: Vec<crate::datadogV2::model::LLMObsAnnotationItemResponse>,
    ) -> LLMObsAnnotationsDataAttributesResponse {
        LLMObsAnnotationsDataAttributesResponse {
            annotations,
            errors: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn errors(mut self, value: Vec<crate::datadogV2::model::LLMObsAnnotationError>) -> Self {
        self.errors = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotationsDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationsDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationsDataAttributesResponseVisitor {
            type Value = LLMObsAnnotationsDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotations: Option<
                    Vec<crate::datadogV2::model::LLMObsAnnotationItemResponse>,
                > = None;
                let mut errors: Option<Vec<crate::datadogV2::model::LLMObsAnnotationError>> = None;
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
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = LLMObsAnnotationsDataAttributesResponse {
                    annotations,
                    errors,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationsDataAttributesResponseVisitor)
    }
}
