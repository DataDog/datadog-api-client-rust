// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a Grok parsing rule, which extracts structured fields from log content using named Grok patterns.
/// Each rule must have a unique name and a valid Datadog Grok pattern that will be applied to the source field.
///
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineParseGrokProcessorRuleMatchRule {
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// The definition of the Grok rule.
    #[serde(rename = "rule")]
    pub rule: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineParseGrokProcessorRuleMatchRule {
    pub fn new(name: String, rule: String) -> ObservabilityPipelineParseGrokProcessorRuleMatchRule {
        ObservabilityPipelineParseGrokProcessorRuleMatchRule {
            name,
            rule,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineParseGrokProcessorRuleMatchRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineParseGrokProcessorRuleMatchRuleVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineParseGrokProcessorRuleMatchRuleVisitor {
            type Value = ObservabilityPipelineParseGrokProcessorRuleMatchRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut rule: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rule = rule.ok_or_else(|| M::Error::missing_field("rule"))?;

                let content = ObservabilityPipelineParseGrokProcessorRuleMatchRule {
                    name,
                    rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineParseGrokProcessorRuleMatchRuleVisitor)
    }
}
