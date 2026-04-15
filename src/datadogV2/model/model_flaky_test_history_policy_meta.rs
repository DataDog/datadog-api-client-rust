// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the policy that triggered this status change.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestHistoryPolicyMeta {
    /// Branches where the test was flaky at the time of the status change.
    #[serde(
        rename = "branches",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub branches: Option<Option<Vec<String>>>,
    /// Configuration parameters of the policy that triggered this status change.
    #[serde(rename = "config")]
    pub config: Option<crate::datadogV2::model::FlakyTestHistoryPolicyMetaConfig>,
    /// The number of days the test has been active at the time of the status change.
    #[serde(
        rename = "days_active",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub days_active: Option<Option<i32>>,
    /// The number of days since the test last exhibited flakiness.
    #[serde(
        rename = "days_without_flake",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub days_without_flake: Option<Option<i32>>,
    /// The failure rate of the test at the time of the status change.
    #[serde(
        rename = "failure_rate",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub failure_rate: Option<Option<f64>>,
    /// The previous state of the test.
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option")]
    pub state: Option<Option<String>>,
    /// The total number of test runs at the time of the status change.
    #[serde(
        rename = "total_runs",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_runs: Option<Option<i32>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestHistoryPolicyMeta {
    pub fn new() -> FlakyTestHistoryPolicyMeta {
        FlakyTestHistoryPolicyMeta {
            branches: None,
            config: None,
            days_active: None,
            days_without_flake: None,
            failure_rate: None,
            state: None,
            total_runs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn branches(mut self, value: Option<Vec<String>>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn config(
        mut self,
        value: crate::datadogV2::model::FlakyTestHistoryPolicyMetaConfig,
    ) -> Self {
        self.config = Some(value);
        self
    }

    pub fn days_active(mut self, value: Option<i32>) -> Self {
        self.days_active = Some(value);
        self
    }

    pub fn days_without_flake(mut self, value: Option<i32>) -> Self {
        self.days_without_flake = Some(value);
        self
    }

    pub fn failure_rate(mut self, value: Option<f64>) -> Self {
        self.failure_rate = Some(value);
        self
    }

    pub fn state(mut self, value: Option<String>) -> Self {
        self.state = Some(value);
        self
    }

    pub fn total_runs(mut self, value: Option<i32>) -> Self {
        self.total_runs = Some(value);
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

impl Default for FlakyTestHistoryPolicyMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestHistoryPolicyMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestHistoryPolicyMetaVisitor;
        impl<'a> Visitor<'a> for FlakyTestHistoryPolicyMetaVisitor {
            type Value = FlakyTestHistoryPolicyMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branches: Option<Option<Vec<String>>> = None;
                let mut config: Option<crate::datadogV2::model::FlakyTestHistoryPolicyMetaConfig> =
                    None;
                let mut days_active: Option<Option<i32>> = None;
                let mut days_without_flake: Option<Option<i32>> = None;
                let mut failure_rate: Option<Option<f64>> = None;
                let mut state: Option<Option<String>> = None;
                let mut total_runs: Option<Option<i32>> = None;
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
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "days_active" => {
                            days_active =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "days_without_flake" => {
                            days_without_flake =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure_rate" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            failure_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_runs" => {
                            total_runs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FlakyTestHistoryPolicyMeta {
                    branches,
                    config,
                    days_active,
                    days_without_flake,
                    failure_rate,
                    state,
                    total_runs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestHistoryPolicyMetaVisitor)
    }
}
