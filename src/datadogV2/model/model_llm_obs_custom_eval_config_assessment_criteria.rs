// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Criteria used to assess the pass/fail result of a custom evaluator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigAssessmentCriteria {
    /// Maximum numeric threshold for a passing result.
    #[serde(
        rename = "max_threshold",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub max_threshold: Option<Option<f64>>,
    /// Minimum numeric threshold for a passing result.
    #[serde(
        rename = "min_threshold",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub min_threshold: Option<Option<f64>>,
    /// Specific output values considered as a passing result.
    #[serde(
        rename = "pass_values",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub pass_values: Option<Option<Vec<String>>>,
    /// When true, a boolean output of true is treated as passing.
    #[serde(
        rename = "pass_when",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub pass_when: Option<Option<bool>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigAssessmentCriteria {
    pub fn new() -> LLMObsCustomEvalConfigAssessmentCriteria {
        LLMObsCustomEvalConfigAssessmentCriteria {
            max_threshold: None,
            min_threshold: None,
            pass_values: None,
            pass_when: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn max_threshold(mut self, value: Option<f64>) -> Self {
        self.max_threshold = Some(value);
        self
    }

    pub fn min_threshold(mut self, value: Option<f64>) -> Self {
        self.min_threshold = Some(value);
        self
    }

    pub fn pass_values(mut self, value: Option<Vec<String>>) -> Self {
        self.pass_values = Some(value);
        self
    }

    pub fn pass_when(mut self, value: Option<bool>) -> Self {
        self.pass_when = Some(value);
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

impl Default for LLMObsCustomEvalConfigAssessmentCriteria {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigAssessmentCriteria {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigAssessmentCriteriaVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigAssessmentCriteriaVisitor {
            type Value = LLMObsCustomEvalConfigAssessmentCriteria;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_threshold: Option<Option<f64>> = None;
                let mut min_threshold: Option<Option<f64>> = None;
                let mut pass_values: Option<Option<Vec<String>>> = None;
                let mut pass_when: Option<Option<bool>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_threshold" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            max_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_threshold" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            min_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pass_values" => {
                            pass_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pass_when" => {
                            pass_when = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsCustomEvalConfigAssessmentCriteria {
                    max_threshold,
                    min_threshold,
                    pass_values,
                    pass_when,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigAssessmentCriteriaVisitor)
    }
}
