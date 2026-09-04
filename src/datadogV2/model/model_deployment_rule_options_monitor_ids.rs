// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Specific monitor options for deployment rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleOptionsMonitorIds {
    /// Seconds the monitors need to stay in OK status for the rule to pass.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Whether the rule should fail if a selected monitor group is in a NO DATA state.
    #[serde(rename = "fail_on_no_data")]
    pub fail_on_no_data: Option<bool>,
    /// Whether the rule should fail if no monitor groups are found for the selected monitors.
    #[serde(rename = "fail_on_no_groups_found")]
    pub fail_on_no_groups_found: Option<bool>,
    /// A non-empty list of specific monitors to evaluate.
    #[serde(rename = "monitor_ids")]
    pub monitor_ids: Vec<crate::datadogV2::model::DeploymentRuleOptionsMonitorId>,
    /// Seconds to wait after a deployment starts before evaluating the monitors' statuses.
    #[serde(rename = "warmup")]
    pub warmup: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleOptionsMonitorIds {
    pub fn new(
        monitor_ids: Vec<crate::datadogV2::model::DeploymentRuleOptionsMonitorId>,
    ) -> DeploymentRuleOptionsMonitorIds {
        DeploymentRuleOptionsMonitorIds {
            duration: None,
            fail_on_no_data: None,
            fail_on_no_groups_found: None,
            monitor_ids,
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

impl<'de> Deserialize<'de> for DeploymentRuleOptionsMonitorIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleOptionsMonitorIdsVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleOptionsMonitorIdsVisitor {
            type Value = DeploymentRuleOptionsMonitorIds;

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
                let mut monitor_ids: Option<
                    Vec<crate::datadogV2::model::DeploymentRuleOptionsMonitorId>,
                > = None;
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
                        "monitor_ids" => {
                            monitor_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let monitor_ids =
                    monitor_ids.ok_or_else(|| M::Error::missing_field("monitor_ids"))?;

                let content = DeploymentRuleOptionsMonitorIds {
                    duration,
                    fail_on_no_data,
                    fail_on_no_groups_found,
                    monitor_ids,
                    warmup,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleOptionsMonitorIdsVisitor)
    }
}
