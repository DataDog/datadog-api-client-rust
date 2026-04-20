// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a custom LLM Observability evaluator configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigAttributes {
    /// Category of the evaluator.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Timestamp when the evaluator configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A Datadog user associated with a custom evaluator configuration.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::LLMObsCustomEvalConfigUser>,
    /// Name of the custom evaluator.
    #[serde(rename = "eval_name")]
    pub eval_name: String,
    /// A Datadog user associated with a custom evaluator configuration.
    #[serde(rename = "last_updated_by")]
    pub last_updated_by: Option<crate::datadogV2::model::LLMObsCustomEvalConfigUser>,
    /// LLM judge configuration for a custom evaluator.
    #[serde(rename = "llm_judge_config")]
    pub llm_judge_config: Option<crate::datadogV2::model::LLMObsCustomEvalConfigLLMJudgeConfig>,
    /// LLM provider configuration for a custom evaluator.
    #[serde(rename = "llm_provider")]
    pub llm_provider: Option<crate::datadogV2::model::LLMObsCustomEvalConfigLLMProvider>,
    /// Target application configuration for a custom evaluator.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::LLMObsCustomEvalConfigTarget>,
    /// Timestamp when the evaluator configuration was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        eval_name: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> LLMObsCustomEvalConfigAttributes {
        LLMObsCustomEvalConfigAttributes {
            category: None,
            created_at,
            created_by: None,
            eval_name,
            last_updated_by: None,
            llm_judge_config: None,
            llm_provider: None,
            target: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn created_by(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigUser,
    ) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn last_updated_by(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigUser,
    ) -> Self {
        self.last_updated_by = Some(value);
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

    pub fn target(mut self, value: crate::datadogV2::model::LLMObsCustomEvalConfigTarget) -> Self {
        self.target = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigAttributesVisitor {
            type Value = LLMObsCustomEvalConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::LLMObsCustomEvalConfigUser> =
                    None;
                let mut eval_name: Option<String> = None;
                let mut last_updated_by: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigUser,
                > = None;
                let mut llm_judge_config: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigLLMJudgeConfig,
                > = None;
                let mut llm_provider: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigLLMProvider,
                > = None;
                let mut target: Option<crate::datadogV2::model::LLMObsCustomEvalConfigTarget> =
                    None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eval_name" => {
                            eval_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let eval_name = eval_name.ok_or_else(|| M::Error::missing_field("eval_name"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = LLMObsCustomEvalConfigAttributes {
                    category,
                    created_at,
                    created_by,
                    eval_name,
                    last_updated_by,
                    llm_judge_config,
                    llm_provider,
                    target,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigAttributesVisitor)
    }
}
