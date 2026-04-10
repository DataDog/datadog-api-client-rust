// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Branch filtering rule for a Flaky Tests Management policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesBranchRule {
    /// List of branches to which the policy applies.
    #[serde(rename = "branches")]
    pub branches: Option<Vec<String>>,
    /// Whether this branch rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// List of branches excluded from the policy.
    #[serde(rename = "excluded_branches")]
    pub excluded_branches: Option<Vec<String>>,
    /// List of test services excluded from the policy.
    #[serde(rename = "excluded_test_services")]
    pub excluded_test_services: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationFlakyTestsManagementPoliciesBranchRule {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesBranchRule {
        TestOptimizationFlakyTestsManagementPoliciesBranchRule {
            branches: None,
            enabled: None,
            excluded_branches: None,
            excluded_test_services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn branches(mut self, value: Vec<String>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn excluded_branches(mut self, value: Vec<String>) -> Self {
        self.excluded_branches = Some(value);
        self
    }

    pub fn excluded_test_services(mut self, value: Vec<String>) -> Self {
        self.excluded_test_services = Some(value);
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesBranchRule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesBranchRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesBranchRuleVisitor;
        impl<'a> Visitor<'a> for TestOptimizationFlakyTestsManagementPoliciesBranchRuleVisitor {
            type Value = TestOptimizationFlakyTestsManagementPoliciesBranchRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branches: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut excluded_branches: Option<Vec<String>> = None;
                let mut excluded_test_services: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "branches" => {
                            if v.is_null() {
                                continue;
                            }
                            branches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "excluded_branches" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_branches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "excluded_test_services" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_test_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationFlakyTestsManagementPoliciesBranchRule {
                    branches,
                    enabled,
                    excluded_branches,
                    excluded_test_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationFlakyTestsManagementPoliciesBranchRuleVisitor)
    }
}
