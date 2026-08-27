// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Monitor options for deployment rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleOptionsMonitor {
    /// Seconds the monitor needs to stay in OK status for the rule to pass.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Whether the rule should fail if a matching monitor group is in a NO DATA state.
    #[serde(rename = "fail_on_no_data")]
    pub fail_on_no_data: Option<bool>,
    /// Whether the rule should fail if no monitor groups are found for the query.
    #[serde(rename = "fail_on_no_groups_found")]
    pub fail_on_no_groups_found: Option<bool>,
    /// Monitors that match this query are evaluated.
    #[serde(rename = "query")]
    pub query: String,
    /// Seconds to wait after a deployment starts before evaluating the monitor's status.
    #[serde(rename = "warmup")]
    pub warmup: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleOptionsMonitor {
    pub fn new(query: String) -> DeploymentRuleOptionsMonitor {
        DeploymentRuleOptionsMonitor {
            duration: None,
            fail_on_no_data: None,
            fail_on_no_groups_found: None,
            query,
            warmup: None,
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn fail_on_no_data(mut self, value: bool) -> Self {
        self.fail_on_no_data = Some(value);
        self
    }

    pub fn fail_on_no_groups_found(mut self, value: bool) -> Self {
        self.fail_on_no_groups_found = Some(value);
        self
    }

    pub fn warmup(mut self, value: i64) -> Self {
        self.warmup = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DeploymentRuleOptionsMonitor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleOptionsMonitorVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleOptionsMonitorVisitor {
            type Value = DeploymentRuleOptionsMonitor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<i64> = None;
                let mut fail_on_no_data: Option<bool> = None;
                let mut fail_on_no_groups_found: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut warmup: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fail_on_no_data" => {
                            if v.is_null() {
                                continue;
                            }
                            fail_on_no_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fail_on_no_groups_found" => {
                            if v.is_null() {
                                continue;
                            }
                            fail_on_no_groups_found =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warmup" => {
                            if v.is_null() {
                                continue;
                            }
                            warmup = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = DeploymentRuleOptionsMonitor {
                    duration,
                    fail_on_no_data,
                    fail_on_no_groups_found,
                    query,
                    warmup,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleOptionsMonitorVisitor)
    }
}
