// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for the quarantined Flaky Tests Management policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesQuarantined {
    /// Automatic quarantine triggering rule based on a time window.
    #[serde(rename = "auto_quarantine_rule")]
    pub auto_quarantine_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoQuarantineRule>,
    /// Branch filtering rule for a Flaky Tests Management policy.
    #[serde(rename = "branch_rule")]
    pub branch_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesBranchRule>,
    /// Whether the quarantined policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Failure-rate-based rule for the quarantined policy.
    #[serde(rename = "failure_rate_rule")]
    pub failure_rate_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TestOptimizationFlakyTestsManagementPoliciesQuarantined {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesQuarantined {
        TestOptimizationFlakyTestsManagementPoliciesQuarantined {
            auto_quarantine_rule: None,
            branch_rule: None,
            enabled: None,
            failure_rate_rule: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_quarantine_rule(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoQuarantineRule,
    ) -> Self {
        self.auto_quarantine_rule = Some(value);
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
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule,
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesQuarantined {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesQuarantined {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesQuarantinedVisitor;
        impl<'a> Visitor<'a> for TestOptimizationFlakyTestsManagementPoliciesQuarantinedVisitor {
            type Value = TestOptimizationFlakyTestsManagementPoliciesQuarantined;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_quarantine_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAutoQuarantineRule> = None;
                let mut branch_rule: Option<
                    crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesBranchRule,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut failure_rate_rule: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_quarantine_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_quarantine_rule =
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

                let content = TestOptimizationFlakyTestsManagementPoliciesQuarantined {
                    auto_quarantine_rule,
                    branch_rule,
                    enabled,
                    failure_rate_rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationFlakyTestsManagementPoliciesQuarantinedVisitor)
    }
}
