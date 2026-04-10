// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for the disabled Flaky Tests Management policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesDisabled {
    /// Automatic disable triggering rule based on a time window and test status.
    #[serde(rename = "auto_disable_rule")]
    pub auto_disable_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule>,
    /// Branch filtering rule for a Flaky Tests Management policy.
    #[serde(rename = "branch_rule")]
    pub branch_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesBranchRule>,
    /// Whether the disabled policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Failure-rate-based rule for the disabled policy.
    #[serde(rename = "failure_rate_rule")]
    pub failure_rate_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TestOptimizationFlakyTestsManagementPoliciesDisabled {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesDisabled {
        TestOptimizationFlakyTestsManagementPoliciesDisabled {
            auto_disable_rule: None,
            branch_rule: None,
            enabled: None,
            failure_rate_rule: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_disable_rule(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule,
    ) -> Self {
        self.auto_disable_rule = Some(value);
        self
    }

    pub fn branch_rule(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesBranchRule,
    ) -> Self {
        self.branch_rule = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn failure_rate_rule(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule,
    ) -> Self {
        self.failure_rate_rule = Some(value);
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesDisabled {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesDisabled {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesDisabledVisitor;
        impl<'a> Visitor<'a> for TestOptimizationFlakyTestsManagementPoliciesDisabledVisitor {
            type Value = TestOptimizationFlakyTestsManagementPoliciesDisabled;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_disable_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule> = None;
                let mut branch_rule: Option<
                    crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesBranchRule,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut failure_rate_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_disable_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_disable_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "branch_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            branch_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_rate_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            failure_rate_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationFlakyTestsManagementPoliciesDisabled {
                    auto_disable_rule,
                    branch_rule,
                    enabled,
                    failure_rate_rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationFlakyTestsManagementPoliciesDisabledVisitor)
    }
}
