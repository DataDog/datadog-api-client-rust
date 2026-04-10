// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Automatic disable triggering rule based on a time window and test status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
    /// Whether this auto-disable rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Test status that the disable policy applies to.
    /// Must be either `active` or `quarantined`.
    #[serde(rename = "status")]
    pub status:
        Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus>,
    /// Time window in seconds over which flakiness is evaluated. Must be greater than 0.
    #[serde(rename = "window_seconds")]
    pub window_seconds: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
        TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
            enabled: None,
            status: None,
            window_seconds: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn window_seconds(mut self, value: i64) -> Self {
        self.window_seconds = Some(value);
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesAutoDisableRuleVisitor;
        impl<'a> Visitor<'a> for TestOptimizationFlakyTestsManagementPoliciesAutoDisableRuleVisitor {
            type Value = TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut status: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabledStatus> = None;
                let mut window_seconds: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "window_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            window_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationFlakyTestsManagementPoliciesAutoDisableRule {
                    enabled,
                    status,
                    window_seconds,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(TestOptimizationFlakyTestsManagementPoliciesAutoDisableRuleVisitor)
    }
}
