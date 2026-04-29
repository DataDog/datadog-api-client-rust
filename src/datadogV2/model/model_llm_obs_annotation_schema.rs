// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema defining the labels for an annotation queue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationSchema {
    /// List of label schema definitions.
    #[serde(rename = "label_schemas")]
    pub label_schemas: Vec<crate::datadogV2::model::LLMObsLabelSchema>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationSchema {
    pub fn new(
        label_schemas: Vec<crate::datadogV2::model::LLMObsLabelSchema>,
    ) -> LLMObsAnnotationSchema {
        LLMObsAnnotationSchema {
            label_schemas,
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

impl<'de> Deserialize<'de> for LLMObsAnnotationSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationSchemaVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationSchemaVisitor {
            type Value = LLMObsAnnotationSchema;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut label_schemas: Option<Vec<crate::datadogV2::model::LLMObsLabelSchema>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "label_schemas" => {
                            label_schemas =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let label_schemas =
                    label_schemas.ok_or_else(|| M::Error::missing_field("label_schemas"))?;

                let content = LLMObsAnnotationSchema {
                    label_schemas,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationSchemaVisitor)
    }
}
