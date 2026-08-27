// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Averaged DORA and delivery metrics computed across the commits and pull requests included in the deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORADeploymentAveragedMetrics {
    /// The averaged change lead time, in seconds.
    #[serde(rename = "change_lead_time")]
    pub change_lead_time: Option<i64>,
    /// The averaged merge time, in seconds.
    #[serde(rename = "merge_time")]
    pub merge_time: Option<i64>,
    /// The averaged review time, in seconds.
    #[serde(rename = "review_time")]
    pub review_time: Option<i64>,
    /// The averaged time to deploy, in seconds.
    #[serde(rename = "time_to_deploy")]
    pub time_to_deploy: Option<i64>,
    /// The averaged time until the pull request was ready for review, in seconds.
    #[serde(rename = "time_to_pr_ready")]
    pub time_to_pr_ready: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORADeploymentAveragedMetrics {
    pub fn new() -> DORADeploymentAveragedMetrics {
        DORADeploymentAveragedMetrics {
            change_lead_time: None,
            merge_time: None,
            review_time: None,
            time_to_deploy: None,
            time_to_pr_ready: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_lead_time(mut self, value: i64) -> Self {
        self.change_lead_time = Some(value);
        self
    }

    pub fn merge_time(mut self, value: i64) -> Self {
        self.merge_time = Some(value);
        self
    }

    pub fn review_time(mut self, value: i64) -> Self {
        self.review_time = Some(value);
        self
    }

    pub fn time_to_deploy(mut self, value: i64) -> Self {
        self.time_to_deploy = Some(value);
        self
    }

    pub fn time_to_pr_ready(mut self, value: i64) -> Self {
        self.time_to_pr_ready = Some(value);
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

impl Default for DORADeploymentAveragedMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DORADeploymentAveragedMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORADeploymentAveragedMetricsVisitor;
        impl<'a> Visitor<'a> for DORADeploymentAveragedMetricsVisitor {
            type Value = DORADeploymentAveragedMetrics;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_lead_time: Option<i64> = None;
                let mut merge_time: Option<i64> = None;
                let mut review_time: Option<i64> = None;
                let mut time_to_deploy: Option<i64> = None;
                let mut time_to_pr_ready: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_lead_time" => {
                            if v.is_null() {
                                continue;
                            }
                            change_lead_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "merge_time" => {
                            if v.is_null() {
                                continue;
                            }
                            merge_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "review_time" => {
                            if v.is_null() {
                                continue;
                            }
                            review_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_deploy" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_deploy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_pr_ready" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_pr_ready =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DORADeploymentAveragedMetrics {
                    change_lead_time,
                    merge_time,
                    review_time,
                    time_to_deploy,
                    time_to_pr_ready,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORADeploymentAveragedMetricsVisitor)
    }
}
