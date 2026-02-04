// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the extra options for a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestOptions {
    /// Minimum amount of time in failure required to trigger an alert.
    #[serde(rename = "min_failure_duration")]
    pub min_failure_duration: Option<i64>,
    /// Minimum number of locations in failure required to trigger
    /// an alert.
    #[serde(rename = "min_location_failed")]
    pub min_location_failed: Option<i64>,
    /// The monitor name is used for the alert title as well as for all monitor dashboard widgets and SLOs.
    #[serde(rename = "monitor_name")]
    pub monitor_name: Option<String>,
    /// Object containing the options for a Synthetic test as a monitor
    /// (for example, renotification).
    #[serde(rename = "monitor_options")]
    pub monitor_options: Option<crate::datadogV2::model::SyntheticsTestOptionsMonitorOptions>,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(rename = "monitor_priority")]
    pub monitor_priority: Option<i32>,
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access. This field is deprecated. Use the restriction policies API to manage permissions.
    #[deprecated]
    #[serde(rename = "restricted_roles")]
    pub restricted_roles: Option<Vec<String>>,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV2::model::SyntheticsTestOptionsRetry>,
    /// Object containing timeframes and timezone used for advanced scheduling.
    #[serde(rename = "scheduling")]
    pub scheduling: Option<crate::datadogV2::model::SyntheticsTestOptionsScheduling>,
    /// The frequency at which to run the Synthetic test (in seconds).
    #[serde(rename = "tick_every")]
    pub tick_every: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestOptions {
    pub fn new() -> SyntheticsTestOptions {
        #[allow(deprecated)]
        SyntheticsTestOptions {
            min_failure_duration: None,
            min_location_failed: None,
            monitor_name: None,
            monitor_options: None,
            monitor_priority: None,
            restricted_roles: None,
            retry: None,
            scheduling: None,
            tick_every: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn min_failure_duration(mut self, value: i64) -> Self {
        self.min_failure_duration = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn min_location_failed(mut self, value: i64) -> Self {
        self.min_location_failed = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn monitor_name(mut self, value: String) -> Self {
        self.monitor_name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn monitor_options(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestOptionsMonitorOptions,
    ) -> Self {
        self.monitor_options = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn monitor_priority(mut self, value: i32) -> Self {
        self.monitor_priority = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn restricted_roles(mut self, value: Vec<String>) -> Self {
        self.restricted_roles = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn retry(mut self, value: crate::datadogV2::model::SyntheticsTestOptionsRetry) -> Self {
        self.retry = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn scheduling(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestOptionsScheduling,
    ) -> Self {
        self.scheduling = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn tick_every(mut self, value: i64) -> Self {
        self.tick_every = Some(value);
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

impl Default for SyntheticsTestOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestOptionsVisitor {
            type Value = SyntheticsTestOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut min_failure_duration: Option<i64> = None;
                let mut min_location_failed: Option<i64> = None;
                let mut monitor_name: Option<String> = None;
                let mut monitor_options: Option<
                    crate::datadogV2::model::SyntheticsTestOptionsMonitorOptions,
                > = None;
                let mut monitor_priority: Option<i32> = None;
                let mut restricted_roles: Option<Vec<String>> = None;
                let mut retry: Option<crate::datadogV2::model::SyntheticsTestOptionsRetry> = None;
                let mut scheduling: Option<
                    crate::datadogV2::model::SyntheticsTestOptionsScheduling,
                > = None;
                let mut tick_every: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "min_failure_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            min_failure_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_location_failed" => {
                            if v.is_null() {
                                continue;
                            }
                            min_location_failed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_name" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_options" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_priority" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_priority =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restricted_roles" => {
                            if v.is_null() {
                                continue;
                            }
                            restricted_roles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retry" => {
                            if v.is_null() {
                                continue;
                            }
                            retry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduling" => {
                            if v.is_null() {
                                continue;
                            }
                            scheduling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tick_every" => {
                            if v.is_null() {
                                continue;
                            }
                            tick_every = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = SyntheticsTestOptions {
                    min_failure_duration,
                    min_location_failed,
                    monitor_name,
                    monitor_options,
                    monitor_priority,
                    restricted_roles,
                    retry,
                    scheduling,
                    tick_every,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestOptionsVisitor)
    }
}
