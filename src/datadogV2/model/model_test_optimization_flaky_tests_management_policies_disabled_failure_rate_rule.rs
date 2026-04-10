// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Failure-rate-based rule for the disabled policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
    /// List of branches to which this rule applies.
    #[serde(rename = "branches")]
    pub branches: Option<Vec<String>>,
    /// Whether this failure rate rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Minimum number of runs required before the rule is evaluated. Must be greater than or equal to 0.
    #[serde(rename = "min_runs")]
    pub min_runs: Option<i64>,
    /// Test status that the disable policy applies to.
    /// Must be either `active` or `quarantined`.
    #[serde(rename = "status")]
    pub status:
        Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus>,
    /// Failure rate threshold (0.0–1.0) above which the rule triggers.
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
        TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
            branches: None,
            enabled: None,
            min_runs: None,
            status: None,
            threshold: None,
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

    pub fn min_runs(mut self, value: i64) -> Self {
        self.min_runs = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRuleVisitor;
        impl<'a> Visitor<'a>
            for TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRuleVisitor
        {
            type Value = TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branches: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut min_runs: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus> = None;
                let mut threshold: Option<f64> = None;
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
                        "min_runs" => {
                            if v.is_null() {
                                continue;
                            }
                            min_runs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "threshold" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            threshold = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRule {
                    branches,
                    enabled,
                    min_runs,
                    status,
                    threshold,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            TestOptimizationFlakyTestsManagementPoliciesDisabledFailureRateRuleVisitor,
        )
    }
}
