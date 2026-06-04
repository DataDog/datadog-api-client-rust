// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single label value entry in an annotation.
/// The `value` type must match the label schema type:
/// - `score`: a number within the schema `min`/`max` range (integer if `is_integer` is `true`).
/// - `categorical`: a string that is one of the schema `values`.
/// - `boolean`: `true` or `false`.
/// - `text`: any non-empty string.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationLabelValue {
    /// Assessment result for a label value.
    #[serde(rename = "assessment")]
    pub assessment: Option<crate::datadogV2::model::LLMObsAnnotationAssessment>,
    /// ID of the label schema this value corresponds to.
    #[serde(rename = "label_schema_id")]
    pub label_schema_id: String,
    /// Free text reasoning for this label value.
    #[serde(rename = "reasoning")]
    pub reasoning: Option<String>,
    /// The value for this label. Must comply with the label schema type constraints.
    #[serde(rename = "value")]
    pub value: crate::datadogV2::model::LLMObsAnnotationLabelValueValue,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationLabelValue {
    pub fn new(
        label_schema_id: String,
        value: crate::datadogV2::model::LLMObsAnnotationLabelValueValue,
    ) -> LLMObsAnnotationLabelValue {
        LLMObsAnnotationLabelValue {
            assessment: None,
            label_schema_id,
            reasoning: None,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assessment(
        mut self,
        value: crate::datadogV2::model::LLMObsAnnotationAssessment,
    ) -> Self {
        self.assessment = Some(value);
        self
    }

    pub fn reasoning(mut self, value: String) -> Self {
        self.reasoning = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotationLabelValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationLabelValueVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationLabelValueVisitor {
            type Value = LLMObsAnnotationLabelValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assessment: Option<crate::datadogV2::model::LLMObsAnnotationAssessment> =
                    None;
                let mut label_schema_id: Option<String> = None;
                let mut reasoning: Option<String> = None;
                let mut value: Option<crate::datadogV2::model::LLMObsAnnotationLabelValueValue> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assessment" => {
                            if v.is_null() {
                                continue;
                            }
                            assessment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _assessment) = assessment {
                                match _assessment {
                                    crate::datadogV2::model::LLMObsAnnotationAssessment::UnparsedObject(_assessment) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "label_schema_id" => {
                            label_schema_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV2::model::LLMObsAnnotationLabelValueValue::UnparsedObject(_value) => {
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
                let label_schema_id =
                    label_schema_id.ok_or_else(|| M::Error::missing_field("label_schema_id"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = LLMObsAnnotationLabelValue {
                    assessment,
                    label_schema_id,
                    reasoning,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationLabelValueVisitor)
    }
}
