// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration parameters of the policy that triggered this status change.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestHistoryPolicyMetaConfig {
    /// The branches considered by the policy.
    #[serde(
        rename = "branches",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub branches: Option<Option<Vec<String>>>,
    /// The number of days a test must have been active for the policy to trigger.
    #[serde(
        rename = "days_active",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub days_active: Option<Option<i32>>,
    /// The failure rate threshold for the policy to trigger.
    #[serde(
        rename = "failure_rate",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub failure_rate: Option<Option<f64>>,
    /// Branches excluded from the policy evaluation.
    #[serde(
        rename = "forget_branches",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub forget_branches: Option<Option<Vec<String>>>,
    /// The minimum number of test runs required for the policy to trigger.
    #[serde(
        rename = "required_runs",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub required_runs: Option<Option<i32>>,
    /// The target state the policy transitions the test from.
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option")]
    pub state: Option<Option<String>>,
    /// Test services excluded from the policy evaluation.
    #[serde(
        rename = "test_services",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub test_services: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestHistoryPolicyMetaConfig {
    pub fn new() -> FlakyTestHistoryPolicyMetaConfig {
        FlakyTestHistoryPolicyMetaConfig {
            branches: None,
            days_active: None,
            failure_rate: None,
            forget_branches: None,
            required_runs: None,
            state: None,
            test_services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn branches(mut self, value: Option<Vec<String>>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn days_active(mut self, value: Option<i32>) -> Self {
        self.days_active = Some(value);
        self
    }

    pub fn failure_rate(mut self, value: Option<f64>) -> Self {
        self.failure_rate = Some(value);
        self
    }

    pub fn forget_branches(mut self, value: Option<Vec<String>>) -> Self {
        self.forget_branches = Some(value);
        self
    }

    pub fn required_runs(mut self, value: Option<i32>) -> Self {
        self.required_runs = Some(value);
        self
    }

    pub fn state(mut self, value: Option<String>) -> Self {
        self.state = Some(value);
        self
    }

    pub fn test_services(mut self, value: Option<Vec<String>>) -> Self {
        self.test_services = Some(value);
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

impl Default for FlakyTestHistoryPolicyMetaConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestHistoryPolicyMetaConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestHistoryPolicyMetaConfigVisitor;
        impl<'a> Visitor<'a> for FlakyTestHistoryPolicyMetaConfigVisitor {
            type Value = FlakyTestHistoryPolicyMetaConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branches: Option<Option<Vec<String>>> = None;
                let mut days_active: Option<Option<i32>> = None;
                let mut failure_rate: Option<Option<f64>> = None;
                let mut forget_branches: Option<Option<Vec<String>>> = None;
                let mut required_runs: Option<Option<i32>> = None;
                let mut state: Option<Option<String>> = None;
                let mut test_services: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "branches" => {
                            branches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "days_active" => {
                            days_active =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_rate" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            failure_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forget_branches" => {
                            forget_branches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required_runs" => {
                            required_runs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_services" => {
                            test_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FlakyTestHistoryPolicyMetaConfig {
                    branches,
                    days_active,
                    failure_rate,
                    forget_branches,
                    required_runs,
                    state,
                    test_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestHistoryPolicyMetaConfigVisitor)
    }
}
