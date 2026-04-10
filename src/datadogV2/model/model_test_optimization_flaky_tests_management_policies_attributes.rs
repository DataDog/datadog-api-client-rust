// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Flaky Tests Management policies for a repository.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationFlakyTestsManagementPoliciesAttributes {
    /// Configuration for the attempt-to-fix Flaky Tests Management policy.
    #[serde(rename = "attempt_to_fix")]
    pub attempt_to_fix:
        Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAttemptToFix>,
    /// Configuration for the disabled Flaky Tests Management policy.
    #[serde(rename = "disabled")]
    pub disabled:
        Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabled>,
    /// Configuration for the quarantined Flaky Tests Management policy.
    #[serde(rename = "quarantined")]
    pub quarantined:
        Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantined>,
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationFlakyTestsManagementPoliciesAttributes {
    pub fn new() -> TestOptimizationFlakyTestsManagementPoliciesAttributes {
        TestOptimizationFlakyTestsManagementPoliciesAttributes {
            attempt_to_fix: None,
            disabled: None,
            quarantined: None,
            repository_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attempt_to_fix(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAttemptToFix,
    ) -> Self {
        self.attempt_to_fix = Some(value);
        self
    }

    pub fn disabled(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabled,
    ) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn quarantined(
        mut self,
        value: crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantined,
    ) -> Self {
        self.quarantined = Some(value);
        self
    }

    pub fn repository_id(mut self, value: String) -> Self {
        self.repository_id = Some(value);
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

impl Default for TestOptimizationFlakyTestsManagementPoliciesAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TestOptimizationFlakyTestsManagementPoliciesAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationFlakyTestsManagementPoliciesAttributesVisitor;
        impl<'a> Visitor<'a> for TestOptimizationFlakyTestsManagementPoliciesAttributesVisitor {
            type Value = TestOptimizationFlakyTestsManagementPoliciesAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attempt_to_fix: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesAttemptToFix> = None;
                let mut disabled: Option<
                    crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesDisabled,
                > = None;
                let mut quarantined: Option<crate::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesQuarantined> = None;
                let mut repository_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attempt_to_fix" => {
                            if v.is_null() {
                                continue;
                            }
                            attempt_to_fix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quarantined" => {
                            if v.is_null() {
                                continue;
                            }
                            quarantined =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TestOptimizationFlakyTestsManagementPoliciesAttributes {
                    attempt_to_fix,
                    disabled,
                    quarantined,
                    repository_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationFlakyTestsManagementPoliciesAttributesVisitor)
    }
}
