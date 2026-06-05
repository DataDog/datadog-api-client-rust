// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single annotation to create or update. The annotation is matched by
/// `interaction_id` and the requesting user's identity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsUpsertAnnotationItem {
    /// ID of the interaction to annotate.
    #[serde(rename = "interaction_id")]
    pub interaction_id: String,
    /// Label values for this annotation. Each entry references a label schema by ID
    /// and provides the corresponding value validated against the schema type constraints.
    #[serde(rename = "label_values")]
    pub label_values: Vec<crate::datadogV2::model::LLMObsAnnotationLabelValue>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsUpsertAnnotationItem {
    pub fn new(
        interaction_id: String,
        label_values: Vec<crate::datadogV2::model::LLMObsAnnotationLabelValue>,
    ) -> LLMObsUpsertAnnotationItem {
        LLMObsUpsertAnnotationItem {
            interaction_id,
            label_values,
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

impl<'de> Deserialize<'de> for LLMObsUpsertAnnotationItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsUpsertAnnotationItemVisitor;
        impl<'a> Visitor<'a> for LLMObsUpsertAnnotationItemVisitor {
            type Value = LLMObsUpsertAnnotationItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interaction_id: Option<String> = None;
                let mut label_values: Option<
                    Vec<crate::datadogV2::model::LLMObsAnnotationLabelValue>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interaction_id" => {
                            interaction_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label_values" => {
                            label_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let interaction_id =
                    interaction_id.ok_or_else(|| M::Error::missing_field("interaction_id"))?;
                let label_values =
                    label_values.ok_or_else(|| M::Error::missing_field("label_values"))?;

                let content = LLMObsUpsertAnnotationItem {
                    interaction_id,
                    label_values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsUpsertAnnotationItemVisitor)
    }
}
