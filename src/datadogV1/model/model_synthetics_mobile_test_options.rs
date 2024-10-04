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
pub struct SyntheticsMobileTestOptions {
    /// The `SyntheticsMobileTestOptions` `allowApplicationCrash`.
    #[serde(rename = "allowApplicationCrash")]
    pub allow_application_crash: Option<bool>,
    /// Array of bindings used for the mobile test.
    #[serde(rename = "bindings")]
    pub bindings: Option<Vec<crate::datadogV1::model::SyntheticsMobileTestBinding>>,
    /// CI/CD options for a Synthetic test.
    #[serde(rename = "ci")]
    pub ci: Option<crate::datadogV1::model::SyntheticsMobileTestCiOptions>,
    /// The `SyntheticsMobileTestOptions` `defaultStepTimeout`.
    #[serde(rename = "defaultStepTimeout")]
    pub default_step_timeout: Option<i32>,
    /// For mobile test, array with the different device IDs used to run the test.
    #[serde(rename = "device_ids")]
    pub device_ids: Option<Vec<String>>,
    /// The `SyntheticsMobileTestOptions` `disableAutoAcceptAlert`.
    #[serde(rename = "disableAutoAcceptAlert")]
    pub disable_auto_accept_alert: Option<bool>,
    /// Minimum amount of time in failure required to trigger an alert.
    #[serde(rename = "min_failure_duration")]
    pub min_failure_duration: Option<i64>,
    /// Mobile application for mobile synthetics test.
    #[serde(rename = "mobileApplication")]
    pub mobile_application: Option<crate::datadogV1::model::SyntheticsMobileTestsMobileApplication>,
    /// The monitor name is used for the alert title as well as for all monitor dashboard widgets and SLOs.
    #[serde(rename = "monitor_name")]
    pub monitor_name: Option<String>,
    /// Object containing the options for a mobile Synthetic test as a monitor
    /// (for example, renotification).
    #[serde(rename = "monitor_options")]
    pub monitor_options: Option<crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptions>,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(rename = "monitor_priority")]
    pub monitor_priority: Option<i32>,
    /// The `SyntheticsMobileTestOptions` `noScreenshot`.
    #[serde(rename = "noScreenshot")]
    pub no_screenshot: Option<bool>,
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access.
    #[serde(rename = "restricted_roles")]
    pub restricted_roles: Option<Vec<String>>,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry>,
    /// Object containing timeframes and timezone used for advanced scheduling.
    #[serde(rename = "scheduling")]
    pub scheduling: Option<crate::datadogV1::model::SyntheticsTestOptionsScheduling>,
    /// The frequency at which to run the Synthetic test (in seconds).
    #[serde(rename = "tick_every")]
    pub tick_every: Option<i64>,
    /// The `SyntheticsMobileTestOptions` `verbosity`.
    #[serde(rename = "verbosity")]
    pub verbosity: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileTestOptions {
    pub fn new() -> SyntheticsMobileTestOptions {
        SyntheticsMobileTestOptions {
            allow_application_crash: None,
            bindings: None,
            ci: None,
            default_step_timeout: None,
            device_ids: None,
            disable_auto_accept_alert: None,
            min_failure_duration: None,
            mobile_application: None,
            monitor_name: None,
            monitor_options: None,
            monitor_priority: None,
            no_screenshot: None,
            restricted_roles: None,
            retry: None,
            scheduling: None,
            tick_every: None,
            verbosity: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_application_crash(mut self, value: bool) -> Self {
        self.allow_application_crash = Some(value);
        self
    }

    pub fn bindings(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsMobileTestBinding>,
    ) -> Self {
        self.bindings = Some(value);
        self
    }

    pub fn ci(mut self, value: crate::datadogV1::model::SyntheticsMobileTestCiOptions) -> Self {
        self.ci = Some(value);
        self
    }

    pub fn default_step_timeout(mut self, value: i32) -> Self {
        self.default_step_timeout = Some(value);
        self
    }

    pub fn device_ids(mut self, value: Vec<String>) -> Self {
        self.device_ids = Some(value);
        self
    }

    pub fn disable_auto_accept_alert(mut self, value: bool) -> Self {
        self.disable_auto_accept_alert = Some(value);
        self
    }

    pub fn min_failure_duration(mut self, value: i64) -> Self {
        self.min_failure_duration = Some(value);
        self
    }

    pub fn mobile_application(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestsMobileApplication,
    ) -> Self {
        self.mobile_application = Some(value);
        self
    }

    pub fn monitor_name(mut self, value: String) -> Self {
        self.monitor_name = Some(value);
        self
    }

    pub fn monitor_options(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptions,
    ) -> Self {
        self.monitor_options = Some(value);
        self
    }

    pub fn monitor_priority(mut self, value: i32) -> Self {
        self.monitor_priority = Some(value);
        self
    }

    pub fn no_screenshot(mut self, value: bool) -> Self {
        self.no_screenshot = Some(value);
        self
    }

    pub fn restricted_roles(mut self, value: Vec<String>) -> Self {
        self.restricted_roles = Some(value);
        self
    }

    pub fn retry(mut self, value: crate::datadogV1::model::SyntheticsTestOptionsRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn scheduling(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsScheduling,
    ) -> Self {
        self.scheduling = Some(value);
        self
    }

    pub fn tick_every(mut self, value: i64) -> Self {
        self.tick_every = Some(value);
        self
    }

    pub fn verbosity(mut self, value: i32) -> Self {
        self.verbosity = Some(value);
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

impl Default for SyntheticsMobileTestOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileTestOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileTestOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileTestOptionsVisitor {
            type Value = SyntheticsMobileTestOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_application_crash: Option<bool> = None;
                let mut bindings: Option<
                    Vec<crate::datadogV1::model::SyntheticsMobileTestBinding>,
                > = None;
                let mut ci: Option<crate::datadogV1::model::SyntheticsMobileTestCiOptions> = None;
                let mut default_step_timeout: Option<i32> = None;
                let mut device_ids: Option<Vec<String>> = None;
                let mut disable_auto_accept_alert: Option<bool> = None;
                let mut min_failure_duration: Option<i64> = None;
                let mut mobile_application: Option<
                    crate::datadogV1::model::SyntheticsMobileTestsMobileApplication,
                > = None;
                let mut monitor_name: Option<String> = None;
                let mut monitor_options: Option<
                    crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptions,
                > = None;
                let mut monitor_priority: Option<i32> = None;
                let mut no_screenshot: Option<bool> = None;
                let mut restricted_roles: Option<Vec<String>> = None;
                let mut retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry> = None;
                let mut scheduling: Option<
                    crate::datadogV1::model::SyntheticsTestOptionsScheduling,
                > = None;
                let mut tick_every: Option<i64> = None;
                let mut verbosity: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowApplicationCrash" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_application_crash =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bindings" => {
                            if v.is_null() {
                                continue;
                            }
                            bindings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci" => {
                            if v.is_null() {
                                continue;
                            }
                            ci = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaultStepTimeout" => {
                            if v.is_null() {
                                continue;
                            }
                            default_step_timeout =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            device_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disableAutoAcceptAlert" => {
                            if v.is_null() {
                                continue;
                            }
                            disable_auto_accept_alert =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_failure_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            min_failure_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobileApplication" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_application =
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
                        "noScreenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            no_screenshot =
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
                        "verbosity" => {
                            if v.is_null() {
                                continue;
                            }
                            verbosity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileTestOptions {
                    allow_application_crash,
                    bindings,
                    ci,
                    default_step_timeout,
                    device_ids,
                    disable_auto_accept_alert,
                    min_failure_duration,
                    mobile_application,
                    monitor_name,
                    monitor_options,
                    monitor_priority,
                    no_screenshot,
                    restricted_roles,
                    retry,
                    scheduling,
                    tick_every,
                    verbosity,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileTestOptionsVisitor)
    }
}
