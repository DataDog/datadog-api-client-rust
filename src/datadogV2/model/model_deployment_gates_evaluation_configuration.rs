// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Inline rule definitions for a deployment gate evaluation. When provided, rules are evaluated
/// directly from this configuration instead of using the preconfigured gate rules.
/// At least one rule is required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGatesEvaluationConfiguration {
    /// Gate-level dry run. When enabled, the rules are evaluated normally but the gate always returns `pass`. The real result is visible in the Datadog UI.
    #[serde(rename = "dry_run")]
    pub dry_run: Option<bool>,
    /// The list of rules to evaluate. At least one rule is required.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::DeploymentGatesEvaluationRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGatesEvaluationConfiguration {
    pub fn new(
        rules: Vec<crate::datadogV2::model::DeploymentGatesEvaluationRule>,
    ) -> DeploymentGatesEvaluationConfiguration {
        DeploymentGatesEvaluationConfiguration {
            dry_run: None,
            rules,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
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

impl<'de> Deserialize<'de> for DeploymentGatesEvaluationConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGatesEvaluationConfigurationVisitor;
        impl<'a> Visitor<'a> for DeploymentGatesEvaluationConfigurationVisitor {
            type Value = DeploymentGatesEvaluationConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<bool> = None;
                let mut rules: Option<Vec<crate::datadogV2::model::DeploymentGatesEvaluationRule>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dry_run" => {
                            if v.is_null() {
                                continue;
                            }
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;

                let content = DeploymentGatesEvaluationConfiguration {
                    dry_run,
                    rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGatesEvaluationConfigurationVisitor)
    }
}
