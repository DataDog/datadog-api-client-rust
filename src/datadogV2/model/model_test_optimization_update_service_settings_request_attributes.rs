// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating Test Optimization service settings.
/// All non-required fields are optional; only provided fields will be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationUpdateServiceSettingsRequestAttributes {
    /// Whether Auto Test Retries are enabled for this service.
    #[serde(rename = "auto_test_retries_enabled")]
    pub auto_test_retries_enabled: Option<bool>,
    /// Whether Code Coverage is enabled for this service.
    #[serde(rename = "code_coverage_enabled")]
    pub code_coverage_enabled: Option<bool>,
    /// Whether Early Flake Detection is enabled for this service.
    #[serde(rename = "early_flake_detection_enabled")]
    pub early_flake_detection_enabled: Option<bool>,
    /// The environment name. If omitted, defaults to `none`.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Whether Failed Test Replay is enabled for this service.
    #[serde(rename = "failed_test_replay_enabled")]
    pub failed_test_replay_enabled: Option<bool>,
    /// Whether PR Comments are enabled for this service.
    #[serde(rename = "pr_comments_enabled")]
    pub pr_comments_enabled: Option<bool>,
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: String,
    /// The service name.
    #[serde(rename = "service_name")]
    pub service_name: String,
    /// Whether Test Impact Analysis is enabled for this service.
    #[serde(rename = "test_impact_analysis_enabled")]
    pub test_impact_analysis_enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationUpdateServiceSettingsRequestAttributes {
    pub fn new(
        repository_id: String,
        service_name: String,
    ) -> TestOptimizationUpdateServiceSettingsRequestAttributes {
        TestOptimizationUpdateServiceSettingsRequestAttributes {
            auto_test_retries_enabled: None,
            code_coverage_enabled: None,
            early_flake_detection_enabled: None,
            env: None,
            failed_test_replay_enabled: None,
            pr_comments_enabled: None,
            repository_id,
            service_name,
            test_impact_analysis_enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_test_retries_enabled(mut self, value: bool) -> Self {
        self.auto_test_retries_enabled = Some(value);
        self
    }

    pub fn code_coverage_enabled(mut self, value: bool) -> Self {
        self.code_coverage_enabled = Some(value);
        self
    }

    pub fn early_flake_detection_enabled(mut self, value: bool) -> Self {
        self.early_flake_detection_enabled = Some(value);
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

    pub fn pr_comments_enabled(mut self, value: bool) -> Self {
        self.pr_comments_enabled = Some(value);
        self
    }

    pub fn test_impact_analysis_enabled(mut self, value: bool) -> Self {
        self.test_impact_analysis_enabled = Some(value);
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

impl<'de> Deserialize<'de> for TestOptimizationUpdateServiceSettingsRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationUpdateServiceSettingsRequestAttributesVisitor;
        impl<'a> Visitor<'a> for TestOptimizationUpdateServiceSettingsRequestAttributesVisitor {
            type Value = TestOptimizationUpdateServiceSettingsRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_test_retries_enabled: Option<bool> = None;
                let mut code_coverage_enabled: Option<bool> = None;
                let mut early_flake_detection_enabled: Option<bool> = None;
                let mut env: Option<String> = None;
                let mut failed_test_replay_enabled: Option<bool> = None;
                let mut pr_comments_enabled: Option<bool> = None;
                let mut repository_id: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut test_impact_analysis_enabled: Option<bool> = None;
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
                        "code_coverage_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            code_coverage_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "early_flake_detection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            early_flake_detection_enabled =
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
                        "pr_comments_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            pr_comments_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let repository_id =
                    repository_id.ok_or_else(|| M::Error::missing_field("repository_id"))?;
                let service_name =
                    service_name.ok_or_else(|| M::Error::missing_field("service_name"))?;

                let content = TestOptimizationUpdateServiceSettingsRequestAttributes {
                    auto_test_retries_enabled,
                    code_coverage_enabled,
                    early_flake_detection_enabled,
                    env,
                    failed_test_replay_enabled,
                    pr_comments_enabled,
                    repository_id,
                    service_name,
                    test_impact_analysis_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationUpdateServiceSettingsRequestAttributesVisitor)
    }
}
