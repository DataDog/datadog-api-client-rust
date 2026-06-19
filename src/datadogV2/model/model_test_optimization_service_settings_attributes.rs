// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for Test Optimization service settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationServiceSettingsAttributes {
    /// Whether Auto Test Retries are enabled for this service.
    #[serde(rename = "auto_test_retries_enabled")]
    pub auto_test_retries_enabled: Option<bool>,
    /// Whether the Auto Test Retries setting is overridden at the service level.
    #[serde(rename = "auto_test_retries_enabled_is_overridden")]
    pub auto_test_retries_enabled_is_overridden: Option<bool>,
    /// Whether Code Coverage is enabled for this service.
    #[serde(rename = "code_coverage_enabled")]
    pub code_coverage_enabled: Option<bool>,
    /// Whether the Code Coverage setting is overridden at the service level.
    #[serde(rename = "code_coverage_enabled_is_overridden")]
    pub code_coverage_enabled_is_overridden: Option<bool>,
    /// Whether Early Flake Detection is enabled for this service.
    #[serde(rename = "early_flake_detection_enabled")]
    pub early_flake_detection_enabled: Option<bool>,
    /// Whether the Early Flake Detection setting is overridden at the service level.
    #[serde(rename = "early_flake_detection_enabled_is_overridden")]
    pub early_flake_detection_enabled_is_overridden: Option<bool>,
    /// The environment name.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Whether Failed Test Replay is enabled for this service.
    #[serde(rename = "failed_test_replay_enabled")]
    pub failed_test_replay_enabled: Option<bool>,
    /// Whether the Failed Test Replay setting is overridden at the service level.
    #[serde(rename = "failed_test_replay_enabled_is_overridden")]
    pub failed_test_replay_enabled_is_overridden: Option<bool>,
    /// Whether PR Comments are enabled. This value reflects the repository-level setting and cannot be overridden at the service level.
    #[serde(rename = "pr_comments_enabled")]
    pub pr_comments_enabled: Option<bool>,
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    /// The service name.
    #[serde(rename = "service_name")]
    pub service_name: Option<String>,
    /// Whether Test Impact Analysis is enabled for this service.
    #[serde(rename = "test_impact_analysis_enabled")]
    pub test_impact_analysis_enabled: Option<bool>,
    /// Whether the Test Impact Analysis setting is overridden at the service level.
    #[serde(rename = "test_impact_analysis_enabled_is_overridden")]
    pub test_impact_analysis_enabled_is_overridden: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationServiceSettingsAttributes {
    pub fn new() -> TestOptimizationServiceSettingsAttributes {
        TestOptimizationServiceSettingsAttributes {
            auto_test_retries_enabled: None,
            auto_test_retries_enabled_is_overridden: None,
            code_coverage_enabled: None,
            code_coverage_enabled_is_overridden: None,
            early_flake_detection_enabled: None,
            early_flake_detection_enabled_is_overridden: None,
            env: None,
            failed_test_replay_enabled: None,
            failed_test_replay_enabled_is_overridden: None,
            pr_comments_enabled: None,
            repository_id: None,
            service_name: None,
            test_impact_analysis_enabled: None,
            test_impact_analysis_enabled_is_overridden: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_test_retries_enabled(mut self, value: bool) -> Self {
        self.auto_test_retries_enabled = Some(value);
        self
    }

    pub fn auto_test_retries_enabled_is_overridden(mut self, value: bool) -> Self {
        self.auto_test_retries_enabled_is_overridden = Some(value);
        self
    }

    pub fn code_coverage_enabled(mut self, value: bool) -> Self {
        self.code_coverage_enabled = Some(value);
        self
    }

    pub fn code_coverage_enabled_is_overridden(mut self, value: bool) -> Self {
        self.code_coverage_enabled_is_overridden = Some(value);
        self
    }

    pub fn early_flake_detection_enabled(mut self, value: bool) -> Self {
        self.early_flake_detection_enabled = Some(value);
        self
    }

    pub fn early_flake_detection_enabled_is_overridden(mut self, value: bool) -> Self {
        self.early_flake_detection_enabled_is_overridden = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn failed_test_replay_enabled(mut self, value: bool) -> Self {
        self.failed_test_replay_enabled = Some(value);
        self
    }

    pub fn failed_test_replay_enabled_is_overridden(mut self, value: bool) -> Self {
        self.failed_test_replay_enabled_is_overridden = Some(value);
        self
    }

    pub fn pr_comments_enabled(mut self, value: bool) -> Self {
        self.pr_comments_enabled = Some(value);
        self
    }

    pub fn repository_id(mut self, value: String) -> Self {
        self.repository_id = Some(value);
        self
    }

    pub fn service_name(mut self, value: String) -> Self {
        self.service_name = Some(value);
        self
    }

    pub fn test_impact_analysis_enabled(mut self, value: bool) -> Self {
        self.test_impact_analysis_enabled = Some(value);
        self
    }

    pub fn test_impact_analysis_enabled_is_overridden(mut self, value: bool) -> Self {
        self.test_impact_analysis_enabled_is_overridden = Some(value);
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

impl Default for TestOptimizationServiceSettingsAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationServiceSettingsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationServiceSettingsAttributesVisitor;
        impl<'a> Visitor<'a> for TestOptimizationServiceSettingsAttributesVisitor {
            type Value = TestOptimizationServiceSettingsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_test_retries_enabled: Option<bool> = None;
                let mut auto_test_retries_enabled_is_overridden: Option<bool> = None;
                let mut code_coverage_enabled: Option<bool> = None;
                let mut code_coverage_enabled_is_overridden: Option<bool> = None;
                let mut early_flake_detection_enabled: Option<bool> = None;
                let mut early_flake_detection_enabled_is_overridden: Option<bool> = None;
                let mut env: Option<String> = None;
                let mut failed_test_replay_enabled: Option<bool> = None;
                let mut failed_test_replay_enabled_is_overridden: Option<bool> = None;
                let mut pr_comments_enabled: Option<bool> = None;
                let mut repository_id: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut test_impact_analysis_enabled: Option<bool> = None;
                let mut test_impact_analysis_enabled_is_overridden: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_test_retries_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_test_retries_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auto_test_retries_enabled_is_overridden" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_test_retries_enabled_is_overridden =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code_coverage_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            code_coverage_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code_coverage_enabled_is_overridden" => {
                            if v.is_null() {
                                continue;
                            }
                            code_coverage_enabled_is_overridden =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "early_flake_detection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            early_flake_detection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "early_flake_detection_enabled_is_overridden" => {
                            if v.is_null() {
                                continue;
                            }
                            early_flake_detection_enabled_is_overridden =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failed_test_replay_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_test_replay_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failed_test_replay_enabled_is_overridden" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_test_replay_enabled_is_overridden =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pr_comments_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            pr_comments_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            if v.is_null() {
                                continue;
                            }
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_impact_analysis_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            test_impact_analysis_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_impact_analysis_enabled_is_overridden" => {
                            if v.is_null() {
                                continue;
                            }
                            test_impact_analysis_enabled_is_overridden =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationServiceSettingsAttributes {
                    auto_test_retries_enabled,
                    auto_test_retries_enabled_is_overridden,
                    code_coverage_enabled,
                    code_coverage_enabled_is_overridden,
                    early_flake_detection_enabled,
                    early_flake_detection_enabled_is_overridden,
                    env,
                    failed_test_replay_enabled,
                    failed_test_replay_enabled_is_overridden,
                    pr_comments_enabled,
                    repository_id,
                    service_name,
                    test_impact_analysis_enabled,
                    test_impact_analysis_enabled_is_overridden,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationServiceSettingsAttributesVisitor)
    }
}
