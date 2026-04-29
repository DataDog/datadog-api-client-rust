// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema definition for a single label in an annotation queue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsLabelSchema {
    /// Description of the label.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether this label includes an assessment field.
    #[serde(rename = "has_assessment")]
    pub has_assessment: Option<bool>,
    /// Whether this label includes a reasoning field.
    #[serde(rename = "has_reasoning")]
    pub has_reasoning: Option<bool>,
    /// Unique identifier of the label schema. Assigned by the server if not provided.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether the boolean label represents an assessment. Requires `has_assessment` to be true.
    #[serde(rename = "is_assessment")]
    pub is_assessment: Option<bool>,
    /// Whether score values must be integers. Applicable to score-type labels.
    #[serde(rename = "is_integer")]
    pub is_integer: Option<bool>,
    /// Whether this label is required for an annotation.
    #[serde(rename = "is_required")]
    pub is_required: Option<bool>,
    /// Maximum value for score-type labels.
    #[serde(rename = "max")]
    pub max: Option<f64>,
    /// Minimum value for score-type labels.
    #[serde(rename = "min")]
    pub min: Option<f64>,
    /// Name of the label. Must match the pattern `^[a-zA-Z0-9_-]+$` and be unique within the queue.
    #[serde(rename = "name")]
    pub name: String,
    /// Type of a label in an annotation queue label schema.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsLabelSchemaType,
    /// Allowed values for categorical-type labels. Must contain at least one non-empty, unique value.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsLabelSchema {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::LLMObsLabelSchemaType,
    ) -> LLMObsLabelSchema {
        LLMObsLabelSchema {
            description: None,
            has_assessment: None,
            has_reasoning: None,
            id: None,
            is_assessment: None,
            is_integer: None,
            is_required: None,
            max: None,
            min: None,
            name,
            type_,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn has_assessment(mut self, value: bool) -> Self {
        self.has_assessment = Some(value);
        self
    }

    pub fn has_reasoning(mut self, value: bool) -> Self {
        self.has_reasoning = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_assessment(mut self, value: bool) -> Self {
        self.is_assessment = Some(value);
        self
    }

    pub fn is_integer(mut self, value: bool) -> Self {
        self.is_integer = Some(value);
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsLabelSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsLabelSchemaVisitor;
        impl<'a> Visitor<'a> for LLMObsLabelSchemaVisitor {
            type Value = LLMObsLabelSchema;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut has_assessment: Option<bool> = None;
                let mut has_reasoning: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut is_assessment: Option<bool> = None;
                let mut is_integer: Option<bool> = None;
                let mut is_required: Option<bool> = None;
                let mut max: Option<f64> = None;
                let mut min: Option<f64> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LLMObsLabelSchemaType> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_assessment" => {
                            if v.is_null() {
                                continue;
                            }
                            has_assessment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            has_reasoning =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_assessment" => {
                            if v.is_null() {
                                continue;
                            }
                            is_assessment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_integer" => {
                            if v.is_null() {
                                continue;
                            }
                            is_integer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_required" => {
                            if v.is_null() {
                                continue;
                            }
                            is_required =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LLMObsLabelSchemaType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsLabelSchema {
                    description,
                    has_assessment,
                    has_reasoning,
                    id,
                    is_assessment,
                    is_integer,
                    is_required,
                    max,
                    min,
                    name,
                    type_,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsLabelSchemaVisitor)
    }
}
