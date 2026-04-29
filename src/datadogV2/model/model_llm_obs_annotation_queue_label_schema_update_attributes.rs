// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an annotation queue label schema.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationQueueLabelSchemaUpdateAttributes {
    /// Schema defining the labels for an annotation queue.
    #[serde(rename = "annotation_schema")]
    pub annotation_schema: crate::datadogV2::model::LLMObsAnnotationSchema,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationQueueLabelSchemaUpdateAttributes {
    pub fn new(
        annotation_schema: crate::datadogV2::model::LLMObsAnnotationSchema,
    ) -> LLMObsAnnotationQueueLabelSchemaUpdateAttributes {
        LLMObsAnnotationQueueLabelSchemaUpdateAttributes {
            annotation_schema,
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

impl<'de> Deserialize<'de> for LLMObsAnnotationQueueLabelSchemaUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationQueueLabelSchemaUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationQueueLabelSchemaUpdateAttributesVisitor {
            type Value = LLMObsAnnotationQueueLabelSchemaUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotation_schema: Option<crate::datadogV2::model::LLMObsAnnotationSchema> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotation_schema" => {
                            annotation_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotation_schema = annotation_schema
                    .ok_or_else(|| M::Error::missing_field("annotation_schema"))?;

                let content = LLMObsAnnotationQueueLabelSchemaUpdateAttributes {
                    annotation_schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationQueueLabelSchemaUpdateAttributesVisitor)
    }
}
