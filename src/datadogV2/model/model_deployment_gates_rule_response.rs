// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The result of a single rule evaluation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGatesRuleResponse {
    /// Whether this rule was evaluated in dry-run mode.
    #[serde(rename = "dry_run")]
    pub dry_run: Option<bool>,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The reason for the rule result, if applicable.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    /// The overall status of the gate evaluation.
    /// - `in_progress`: The evaluation is still running.
    /// - `pass`: All rules passed successfully and the deployment is allowed to proceed.
    /// - `fail`: One or more rules did not pass; the deployment should not proceed.
    #[serde(rename = "status")]
    pub status: Option<
        crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGatesRuleResponse {
    pub fn new() -> DeploymentGatesRuleResponse {
        DeploymentGatesRuleResponse {
            dry_run: None,
            name: None,
            reason: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
    ) -> Self {
        self.status = Some(value);
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

impl Default for DeploymentGatesRuleResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeploymentGatesRuleResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGatesRuleResponseVisitor;
        impl<'a> Visitor<'a> for DeploymentGatesRuleResponseVisitor {
            type Value = DeploymentGatesRuleResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut reason: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus> = None;
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
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus::UnparsedObject(_status) => {
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

                let content = DeploymentGatesRuleResponse {
                    dry_run,
                    name,
                    reason,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGatesRuleResponseVisitor)
    }
}
