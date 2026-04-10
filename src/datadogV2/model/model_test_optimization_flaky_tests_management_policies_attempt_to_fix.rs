// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for the attempt-to-fix Flaky Tests Management policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
    /// Number of retries when attempting to fix a flaky test. Must be greater than 0.
    #[serde(rename = "retries")]
    pub retries: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
        TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
            retries: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn retries(mut self, value: i64) -> Self {
        self.retries = Some(value);
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesAttemptToFixVisitor;
        impl<'a> Visitor<'a> for TestOptimizationFlakyTestsManagementPoliciesAttemptToFixVisitor {
            type Value = TestOptimizationFlakyTestsManagementPoliciesAttemptToFix;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut retries: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "retries" => {
                            if v.is_null() {
                                continue;
                            }
                            retries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationFlakyTestsManagementPoliciesAttemptToFix {
                    retries,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(TestOptimizationFlakyTestsManagementPoliciesAttemptToFixVisitor)
    }
}
