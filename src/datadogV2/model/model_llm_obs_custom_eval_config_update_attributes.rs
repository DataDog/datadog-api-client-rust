// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a custom LLM Observability evaluator configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigUpdateAttributes {
    /// Category of the evaluator.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Name of the custom evaluator. If provided, must match the eval_name path parameter.
    #[serde(rename = "eval_name")]
    pub eval_name: Option<String>,
    /// LLM judge configuration for a custom evaluator.
    #[serde(rename = "llm_judge_config")]
    pub llm_judge_config: Option<crate::datadogV2::model::LLMObsCustomEvalConfigLLMJudgeConfig>,
    /// LLM provider configuration for a custom evaluator.
    #[serde(rename = "llm_provider")]
    pub llm_provider: Option<crate::datadogV2::model::LLMObsCustomEvalConfigLLMProvider>,
    /// Target application configuration for a custom evaluator.
    #[serde(rename = "target")]
    pub target: crate::datadogV2::model::LLMObsCustomEvalConfigTarget,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigUpdateAttributes {
    pub fn new(
        target: crate::datadogV2::model::LLMObsCustomEvalConfigTarget,
    ) -> LLMObsCustomEvalConfigUpdateAttributes {
        LLMObsCustomEvalConfigUpdateAttributes {
            category: None,
            eval_name: None,
            llm_judge_config: None,
            llm_provider: None,
            target,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn eval_name(mut self, value: String) -> Self {
        self.eval_name = Some(value);
        self
    }

    pub fn llm_judge_config(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigLLMJudgeConfig,
    ) -> Self {
        self.llm_judge_config = Some(value);
        self
    }

    pub fn llm_provider(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigLLMProvider,
    ) -> Self {
        self.llm_provider = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigUpdateAttributesVisitor {
            type Value = LLMObsCustomEvalConfigUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut eval_name: Option<String> = None;
                let mut llm_judge_config: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigLLMJudgeConfig,
                > = None;
                let mut llm_provider: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigLLMProvider,
                > = None;
                let mut target: Option<crate::datadogV2::model::LLMObsCustomEvalConfigTarget> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eval_name" => {
                            if v.is_null() {
                                continue;
                            }
                            eval_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "llm_judge_config" => {
                            if v.is_null() {
                                continue;
                            }
                            llm_judge_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "llm_provider" => {
                            if v.is_null() {
                                continue;
                            }
                            llm_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = LLMObsCustomEvalConfigUpdateAttributes {
                    category,
                    eval_name,
                    llm_judge_config,
                    llm_provider,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigUpdateAttributesVisitor)
    }
}
