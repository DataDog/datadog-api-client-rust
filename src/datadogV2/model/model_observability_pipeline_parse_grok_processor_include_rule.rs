// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Grok parsing rule selected using the `include` query. Each rule defines how to extract structured fields
/// from logs matching a Datadog search query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineParseGrokProcessorIncludeRule {
    /// A Datadog search query used to determine which logs this Grok rule targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of Grok parsing rules that define how to extract fields from matching logs.
    /// Each rule must contain a name and a valid Grok pattern.
    #[serde(rename = "match_rules")]
    pub match_rules:
        Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleMatchRule>,
    /// A list of Grok helper rules that can be referenced by the parsing rules.
    #[serde(rename = "support_rules")]
    pub support_rules: Option<
        Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleSupportRule>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineParseGrokProcessorIncludeRule {
    pub fn new(
        include: String,
        match_rules: Vec<
            crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleMatchRule,
        >,
    ) -> ObservabilityPipelineParseGrokProcessorIncludeRule {
        ObservabilityPipelineParseGrokProcessorIncludeRule {
            include,
            match_rules,
            support_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn support_rules(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleSupportRule>,
    ) -> Self {
        self.support_rules = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineParseGrokProcessorIncludeRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineParseGrokProcessorIncludeRuleVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineParseGrokProcessorIncludeRuleVisitor {
            type Value = ObservabilityPipelineParseGrokProcessorIncludeRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include: Option<String> = None;
                let mut match_rules: Option<Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleMatchRule>> = None;
                let mut support_rules: Option<Vec<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleSupportRule>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_rules" => {
                            match_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "support_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            support_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let match_rules =
                    match_rules.ok_or_else(|| M::Error::missing_field("match_rules"))?;

                let content = ObservabilityPipelineParseGrokProcessorIncludeRule {
                    include,
                    match_rules,
                    support_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineParseGrokProcessorIncludeRuleVisitor)
    }
}
