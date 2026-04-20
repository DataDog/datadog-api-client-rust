// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// LLM judge configuration for a custom evaluator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigLLMJudgeConfig {
    /// Criteria used to assess the pass/fail result of a custom evaluator.
    #[serde(rename = "assessment_criteria")]
    pub assessment_criteria:
        Option<crate::datadogV2::model::LLMObsCustomEvalConfigAssessmentCriteria>,
    /// LLM inference parameters for a custom evaluator.
    #[serde(rename = "inference_params")]
    pub inference_params: crate::datadogV2::model::LLMObsCustomEvalConfigInferenceParams,
    /// Name of the last library prompt template used.
    #[serde(
        rename = "last_used_library_prompt_template_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_used_library_prompt_template_name: Option<Option<String>>,
    /// Whether the library prompt template was modified.
    #[serde(
        rename = "modified_library_prompt_template",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_library_prompt_template: Option<Option<bool>>,
    /// JSON schema describing the expected output format of the LLM judge.
    #[serde(
        rename = "output_schema",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub output_schema: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Output parsing type for a custom LLM judge evaluator.
    #[serde(rename = "parsing_type")]
    pub parsing_type: Option<crate::datadogV2::model::LLMObsCustomEvalConfigParsingType>,
    /// List of messages forming the LLM judge prompt template.
    #[serde(rename = "prompt_template")]
    pub prompt_template: Option<Vec<crate::datadogV2::model::LLMObsCustomEvalConfigPromptMessage>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigLLMJudgeConfig {
    pub fn new(
        inference_params: crate::datadogV2::model::LLMObsCustomEvalConfigInferenceParams,
    ) -> LLMObsCustomEvalConfigLLMJudgeConfig {
        LLMObsCustomEvalConfigLLMJudgeConfig {
            assessment_criteria: None,
            inference_params,
            last_used_library_prompt_template_name: None,
            modified_library_prompt_template: None,
            output_schema: None,
            parsing_type: None,
            prompt_template: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assessment_criteria(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigAssessmentCriteria,
    ) -> Self {
        self.assessment_criteria = Some(value);
        self
    }

    pub fn last_used_library_prompt_template_name(mut self, value: Option<String>) -> Self {
        self.last_used_library_prompt_template_name = Some(value);
        self
    }

    pub fn modified_library_prompt_template(mut self, value: Option<bool>) -> Self {
        self.modified_library_prompt_template = Some(value);
        self
    }

    pub fn output_schema(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn parsing_type(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigParsingType,
    ) -> Self {
        self.parsing_type = Some(value);
        self
    }

    pub fn prompt_template(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsCustomEvalConfigPromptMessage>,
    ) -> Self {
        self.prompt_template = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigLLMJudgeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigLLMJudgeConfigVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigLLMJudgeConfigVisitor {
            type Value = LLMObsCustomEvalConfigLLMJudgeConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assessment_criteria: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigAssessmentCriteria,
                > = None;
                let mut inference_params: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigInferenceParams,
                > = None;
                let mut last_used_library_prompt_template_name: Option<Option<String>> = None;
                let mut modified_library_prompt_template: Option<Option<bool>> = None;
                let mut output_schema: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut parsing_type: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigParsingType,
                > = None;
                let mut prompt_template: Option<
                    Vec<crate::datadogV2::model::LLMObsCustomEvalConfigPromptMessage>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assessment_criteria" => {
                            if v.is_null() {
                                continue;
                            }
                            assessment_criteria =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inference_params" => {
                            inference_params =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_used_library_prompt_template_name" => {
                            last_used_library_prompt_template_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_library_prompt_template" => {
                            modified_library_prompt_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_schema" => {
                            output_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parsing_type" => {
                            if v.is_null() {
                                continue;
                            }
                            parsing_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _parsing_type) = parsing_type {
                                match _parsing_type {
                                    crate::datadogV2::model::LLMObsCustomEvalConfigParsingType::UnparsedObject(_parsing_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "prompt_template" => {
                            if v.is_null() {
                                continue;
                            }
                            prompt_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let inference_params =
                    inference_params.ok_or_else(|| M::Error::missing_field("inference_params"))?;

                let content = LLMObsCustomEvalConfigLLMJudgeConfig {
                    assessment_criteria,
                    inference_params,
                    last_used_library_prompt_template_name,
                    modified_library_prompt_template,
                    output_schema,
                    parsing_type,
                    prompt_template,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigLLMJudgeConfigVisitor)
    }
}
