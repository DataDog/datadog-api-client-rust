// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Failure-rate-based rule for the quarantined policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule {
    /// List of branches to which this rule applies.
    #[serde(rename = "branches")]
    pub branches: Option<Vec<String>>,
    /// Whether this failure rate rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Minimum number of runs required before the rule is evaluated. Must be greater than or equal to 0.
    #[serde(rename = "min_runs")]
    pub min_runs: Option<i64>,
    /// Failure rate threshold (0.0–1.0) above which the rule triggers.
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule {
        TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule {
            branches: None,
            enabled: None,
            min_runs: None,
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRuleVisitor;
        impl<'a> Visitor<'a>
            for TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRuleVisitor
        {
            type Value = TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule;

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

                let content =
                    TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRule {
                        branches,
                        enabled,
                        min_runs,
                        threshold,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            TestOptimizationFlakyTestsManagementPoliciesQuarantinedFailureRateRuleVisitor,
        )
    }
}
