// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the annotation deletion response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDeleteAnnotationsDataAttributesResponse {
    /// IDs of the successfully deleted annotations.
    #[serde(rename = "annotation_ids")]
    pub annotation_ids: Vec<String>,
    /// Errors for annotations that could not be deleted.
    #[serde(rename = "errors")]
    pub errors: Vec<crate::datadogV2::model::LLMObsDeleteAnnotationError>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDeleteAnnotationsDataAttributesResponse {
    pub fn new(
        annotation_ids: Vec<String>,
        errors: Vec<crate::datadogV2::model::LLMObsDeleteAnnotationError>,
    ) -> LLMObsDeleteAnnotationsDataAttributesResponse {
        LLMObsDeleteAnnotationsDataAttributesResponse {
            annotation_ids,
            errors,
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

impl<'de> Deserialize<'de> for LLMObsDeleteAnnotationsDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDeleteAnnotationsDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsDeleteAnnotationsDataAttributesResponseVisitor {
            type Value = LLMObsDeleteAnnotationsDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotation_ids: Option<Vec<String>> = None;
                let mut errors: Option<Vec<crate::datadogV2::model::LLMObsDeleteAnnotationError>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotation_ids" => {
                            annotation_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotation_ids =
                    annotation_ids.ok_or_else(|| M::Error::missing_field("annotation_ids"))?;
                let errors = errors.ok_or_else(|| M::Error::missing_field("errors"))?;

                let content = LLMObsDeleteAnnotationsDataAttributesResponse {
                    annotation_ids,
                    errors,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDeleteAnnotationsDataAttributesResponseVisitor)
    }
}
