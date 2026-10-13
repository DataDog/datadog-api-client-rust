// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Evaluated rule configuration. Fields depend on rule type and unset fields are omitted.
/// Monitor rules can include `duration`, `query`, `monitor_ids`, `warmup`, `fail_on_no_groups_found`, and `fail_on_no_data`.
/// Faulty deployment detection rules can include `duration`, `allowed_resources`, and `excluded_resources`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGateRuleEvaluationConfiguration {
    /// APM resources explicitly allowed by faulty deployment detection.
    #[serde(rename = "allowed_resources")]
    pub allowed_resources: Option<Vec<String>>,
    /// Evaluation duration configured for this rule.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// APM resources excluded from faulty deployment detection.
    #[serde(rename = "excluded_resources")]
    pub excluded_resources: Option<Vec<String>>,
    /// Whether a monitor rule fails when no data is found.
    #[serde(rename = "fail_on_no_data")]
    pub fail_on_no_data: Option<bool>,
    /// Whether a monitor rule fails when no groups are found.
    #[serde(rename = "fail_on_no_groups_found")]
    pub fail_on_no_groups_found: Option<bool>,
    /// Monitor IDs evaluated by a monitor rule.
    #[serde(rename = "monitor_ids")]
    pub monitor_ids: Option<Vec<String>>,
    /// Monitor query used by a monitor rule.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Warm-up duration in seconds for a monitor rule. Omitted when zero.
    #[serde(rename = "warmup")]
    pub warmup: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGateRuleEvaluationConfiguration {
    pub fn new() -> DeploymentGateRuleEvaluationConfiguration {
        DeploymentGateRuleEvaluationConfiguration {
            allowed_resources: None,
            duration: None,
            excluded_resources: None,
            fail_on_no_data: None,
            fail_on_no_groups_found: None,
            monitor_ids: None,
            query: None,
            warmup: None,
            _unparsed: false,
        }
    }

    pub fn allowed_resources(mut self, value: Vec<String>) -> Self {
        self.allowed_resources = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn excluded_resources(mut self, value: Vec<String>) -> Self {
        self.excluded_resources = Some(value);
        self
    }

    pub fn fail_on_no_data(mut self, value: bool) -> Self {
        self.fail_on_no_data = Some(value);
        self
    }

    pub fn fail_on_no_groups_found(mut self, value: bool) -> Self {
        self.fail_on_no_groups_found = Some(value);
        self
    }

    pub fn monitor_ids(mut self, value: Vec<String>) -> Self {
        self.monitor_ids = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn warmup(mut self, value: i64) -> Self {
        self.warmup = Some(value);
        self
    }
}

impl Default for DeploymentGateRuleEvaluationConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeploymentGateRuleEvaluationConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGateRuleEvaluationConfigurationVisitor;
        impl<'a> Visitor<'a> for DeploymentGateRuleEvaluationConfigurationVisitor {
            type Value = DeploymentGateRuleEvaluationConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allowed_resources: Option<Vec<String>> = None;
                let mut duration: Option<i64> = None;
                let mut excluded_resources: Option<Vec<String>> = None;
                let mut fail_on_no_data: Option<bool> = None;
                let mut fail_on_no_groups_found: Option<bool> = None;
                let mut monitor_ids: Option<Vec<String>> = None;
                let mut query: Option<String> = None;
                let mut warmup: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowed_resources" => {
                            if v.is_null() {
                                continue;
                            }
                            allowed_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "excluded_resources" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fail_on_no_data" => {
                            if v.is_null() {
                                continue;
                            }
                            fail_on_no_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fail_on_no_groups_found" => {
                            if v.is_null() {
                                continue;
                            }
                            fail_on_no_groups_found =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warmup" => {
                            if v.is_null() {
                                continue;
                            }
                            warmup = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = DeploymentGateRuleEvaluationConfiguration {
                    allowed_resources,
                    duration,
                    excluded_resources,
                    fail_on_no_data,
                    fail_on_no_groups_found,
                    monitor_ids,
                    query,
                    warmup,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGateRuleEvaluationConfigurationVisitor)
    }
}
