// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Target application configuration for a custom evaluator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigTarget {
    /// Name of the ML application this evaluator targets.
    #[serde(rename = "application_name")]
    pub application_name: String,
    /// Whether the evaluator is active for the target application.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Scope at which to evaluate spans.
    #[serde(rename = "eval_scope")]
    pub eval_scope: Option<crate::datadogV2::model::LLMObsCustomEvalConfigEvalScope>,
    /// Filter expression to select which spans to evaluate.
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option")]
    pub filter: Option<Option<String>>,
    /// When true, only root spans are evaluated.
    #[serde(
        rename = "root_spans_only",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub root_spans_only: Option<Option<bool>>,
    /// Percentage of traces to evaluate. Must be greater than 0 and at most 100.
    #[serde(
        rename = "sampling_percentage",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub sampling_percentage: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigTarget {
    pub fn new(application_name: String, enabled: bool) -> LLMObsCustomEvalConfigTarget {
        LLMObsCustomEvalConfigTarget {
            application_name,
            enabled,
            eval_scope: None,
            filter: None,
            root_spans_only: None,
            sampling_percentage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn eval_scope(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigEvalScope,
    ) -> Self {
        self.eval_scope = Some(value);
        self
    }

    pub fn filter(mut self, value: Option<String>) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn root_spans_only(mut self, value: Option<bool>) -> Self {
        self.root_spans_only = Some(value);
        self
    }

    pub fn sampling_percentage(mut self, value: Option<f64>) -> Self {
        self.sampling_percentage = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigTargetVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigTargetVisitor {
            type Value = LLMObsCustomEvalConfigTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_name: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut eval_scope: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigEvalScope,
                > = None;
                let mut filter: Option<Option<String>> = None;
                let mut root_spans_only: Option<Option<bool>> = None;
                let mut sampling_percentage: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_name" => {
                            application_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eval_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            eval_scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _eval_scope) = eval_scope {
                                match _eval_scope {
                                    crate::datadogV2::model::LLMObsCustomEvalConfigEvalScope::UnparsedObject(_eval_scope) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "root_spans_only" => {
                            root_spans_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sampling_percentage" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            sampling_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_name =
                    application_name.ok_or_else(|| M::Error::missing_field("application_name"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;

                let content = LLMObsCustomEvalConfigTarget {
                    application_name,
                    enabled,
                    eval_scope,
                    filter,
                    root_spans_only,
                    sampling_percentage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigTargetVisitor)
    }
}
