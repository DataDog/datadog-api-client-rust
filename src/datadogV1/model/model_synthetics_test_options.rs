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
    /// For SSL test, whether or not the test should allow self signed
    /// certificates.
    #[serde(rename = "accept_self_signed")]
    pub accept_self_signed: Option<bool>,
    /// Allows loading insecure content for an HTTP request in an API test.
    #[serde(rename = "allow_insecure")]
    pub allow_insecure: Option<bool>,
    /// For SSL test, whether or not the test should fail on revoked certificate in stapled OCSP.
    #[serde(rename = "checkCertificateRevocation")]
    pub check_certificate_revocation: Option<bool>,
    /// CI/CD options for a Synthetic test.
    #[serde(rename = "ci")]
    pub ci: Option<crate::datadogV1::model::SyntheticsTestCiOptions>,
    /// For browser test, array with the different device IDs used to run the test.
    #[serde(rename = "device_ids")]
    pub device_ids: Option<Vec<crate::datadogV1::model::SyntheticsDeviceID>>,
    /// Whether or not to disable CORS mechanism.
    #[serde(rename = "disableCors")]
    pub disable_cors: Option<bool>,
    /// Disable Content Security Policy for browser tests.
    #[serde(rename = "disableCsp")]
    pub disable_csp: Option<bool>,
    /// For API HTTP test, whether or not the test should follow redirects.
    #[serde(rename = "follow_redirects")]
    pub follow_redirects: Option<bool>,
    /// HTTP version to use for a Synthetic test.
    #[serde(rename = "httpVersion")]
    pub http_version: Option<crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion>,
    /// Ignore server certificate error for browser tests.
    #[serde(rename = "ignoreServerCertificateError")]
    pub ignore_server_certificate_error: Option<bool>,
    /// Timeout before declaring the initial step as failed (in seconds) for browser tests.
    #[serde(rename = "initialNavigationTimeout")]
    pub initial_navigation_timeout: Option<i64>,
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
    pub monitor_options: Option<crate::datadogV1::model::SyntheticsTestOptionsMonitorOptions>,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(rename = "monitor_priority")]
    pub monitor_priority: Option<i32>,
    /// Prevents saving screenshots of the steps.
    #[serde(rename = "noScreenshot")]
    pub no_screenshot: Option<bool>,
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access.
    #[serde(rename = "restricted_roles")]
    pub restricted_roles: Option<Vec<String>>,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry>,
    /// The RUM data collection settings for the Synthetic browser test.
    /// **Note:** There are 3 ways to format RUM settings:
    ///
    /// `{ isEnabled: false }`
    /// RUM data is not collected.
    ///
    /// `{ isEnabled: true }`
    /// RUM data is collected from the Synthetic test's default application.
    ///
    /// `{ isEnabled: true, applicationId: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", clientTokenId: 12345 }`
    /// RUM data is collected using the specified application.
    #[serde(rename = "rumSettings")]
    pub rum_settings: Option<crate::datadogV1::model::SyntheticsBrowserTestRumSettings>,
    /// Object containing timeframes and timezone used for advanced scheduling.
    #[serde(rename = "scheduling")]
    pub scheduling: Option<crate::datadogV1::model::SyntheticsTestOptionsScheduling>,
    /// The frequency at which to run the Synthetic test (in seconds).
    #[serde(rename = "tick_every")]
    pub tick_every: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestOptions {
    pub fn new() -> SyntheticsTestOptions {
        SyntheticsTestOptions {
            accept_self_signed: None,
            allow_insecure: None,
            check_certificate_revocation: None,
            ci: None,
            device_ids: None,
            disable_cors: None,
            disable_csp: None,
            follow_redirects: None,
            http_version: None,
            ignore_server_certificate_error: None,
            initial_navigation_timeout: None,
            min_failure_duration: None,
            min_location_failed: None,
            monitor_name: None,
            monitor_options: None,
            monitor_priority: None,
            no_screenshot: None,
            restricted_roles: None,
            retry: None,
            rum_settings: None,
            scheduling: None,
            tick_every: None,
            _unparsed: false,
        }
    }

    pub fn accept_self_signed(&mut self, value: bool) -> &mut Self {
        self.accept_self_signed = Some(value);
        self
    }

    pub fn allow_insecure(&mut self, value: bool) -> &mut Self {
        self.allow_insecure = Some(value);
        self
    }

    pub fn check_certificate_revocation(&mut self, value: bool) -> &mut Self {
        self.check_certificate_revocation = Some(value);
        self
    }

    pub fn ci(&mut self, value: crate::datadogV1::model::SyntheticsTestCiOptions) -> &mut Self {
        self.ci = Some(value);
        self
    }

    pub fn device_ids(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsDeviceID>,
    ) -> &mut Self {
        self.device_ids = Some(value);
        self
    }

    pub fn disable_cors(&mut self, value: bool) -> &mut Self {
        self.disable_cors = Some(value);
        self
    }

    pub fn disable_csp(&mut self, value: bool) -> &mut Self {
        self.disable_csp = Some(value);
        self
    }

    pub fn follow_redirects(&mut self, value: bool) -> &mut Self {
        self.follow_redirects = Some(value);
        self
    }

    pub fn http_version(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion,
    ) -> &mut Self {
        self.http_version = Some(value);
        self
    }

    pub fn ignore_server_certificate_error(&mut self, value: bool) -> &mut Self {
        self.ignore_server_certificate_error = Some(value);
        self
    }

    pub fn initial_navigation_timeout(&mut self, value: i64) -> &mut Self {
        self.initial_navigation_timeout = Some(value);
        self
    }

    pub fn min_failure_duration(&mut self, value: i64) -> &mut Self {
        self.min_failure_duration = Some(value);
        self
    }

    pub fn min_location_failed(&mut self, value: i64) -> &mut Self {
        self.min_location_failed = Some(value);
        self
    }

    pub fn monitor_name(&mut self, value: String) -> &mut Self {
        self.monitor_name = Some(value);
        self
    }

    pub fn monitor_options(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsMonitorOptions,
    ) -> &mut Self {
        self.monitor_options = Some(value);
        self
    }

    pub fn monitor_priority(&mut self, value: i32) -> &mut Self {
        self.monitor_priority = Some(value);
        self
    }

    pub fn no_screenshot(&mut self, value: bool) -> &mut Self {
        self.no_screenshot = Some(value);
        self
    }

    pub fn restricted_roles(&mut self, value: Vec<String>) -> &mut Self {
        self.restricted_roles = Some(value);
        self
    }

    pub fn retry(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsRetry,
    ) -> &mut Self {
        self.retry = Some(value);
        self
    }

    pub fn rum_settings(
        &mut self,
        value: crate::datadogV1::model::SyntheticsBrowserTestRumSettings,
    ) -> &mut Self {
        self.rum_settings = Some(value);
        self
    }

    pub fn scheduling(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsScheduling,
    ) -> &mut Self {
        self.scheduling = Some(value);
        self
    }

    pub fn tick_every(&mut self, value: i64) -> &mut Self {
        self.tick_every = Some(value);
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
                let mut accept_self_signed: Option<bool> = None;
                let mut allow_insecure: Option<bool> = None;
                let mut check_certificate_revocation: Option<bool> = None;
                let mut ci: Option<crate::datadogV1::model::SyntheticsTestCiOptions> = None;
                let mut device_ids: Option<Vec<crate::datadogV1::model::SyntheticsDeviceID>> = None;
                let mut disable_cors: Option<bool> = None;
                let mut disable_csp: Option<bool> = None;
                let mut follow_redirects: Option<bool> = None;
                let mut http_version: Option<
                    crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion,
                > = None;
                let mut ignore_server_certificate_error: Option<bool> = None;
                let mut initial_navigation_timeout: Option<i64> = None;
                let mut min_failure_duration: Option<i64> = None;
                let mut min_location_failed: Option<i64> = None;
                let mut monitor_name: Option<String> = None;
                let mut monitor_options: Option<
                    crate::datadogV1::model::SyntheticsTestOptionsMonitorOptions,
                > = None;
                let mut monitor_priority: Option<i32> = None;
                let mut no_screenshot: Option<bool> = None;
                let mut restricted_roles: Option<Vec<String>> = None;
                let mut retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry> = None;
                let mut rum_settings: Option<
                    crate::datadogV1::model::SyntheticsBrowserTestRumSettings,
                > = None;
                let mut scheduling: Option<
                    crate::datadogV1::model::SyntheticsTestOptionsScheduling,
                > = None;
                let mut tick_every: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accept_self_signed" => {
                            if v.is_null() {
                                continue;
                            }
                            accept_self_signed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allow_insecure" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_insecure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "checkCertificateRevocation" => {
                            if v.is_null() {
                                continue;
                            }
                            check_certificate_revocation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci" => {
                            if v.is_null() {
                                continue;
                            }
                            ci = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            device_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disableCors" => {
                            if v.is_null() {
                                continue;
                            }
                            disable_cors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disableCsp" => {
                            if v.is_null() {
                                continue;
                            }
                            disable_csp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "follow_redirects" => {
                            if v.is_null() {
                                continue;
                            }
                            follow_redirects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "httpVersion" => {
                            if v.is_null() {
                                continue;
                            }
                            http_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _http_version) = http_version {
                                match _http_version {
                                    crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion::UnparsedObject(_http_version) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "ignoreServerCertificateError" => {
                            if v.is_null() {
                                continue;
                            }
                            ignore_server_certificate_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initialNavigationTimeout" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_navigation_timeout =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "rumSettings" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_settings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }

                let content = SyntheticsTestOptions {
                    accept_self_signed,
                    allow_insecure,
                    check_certificate_revocation,
                    ci,
                    device_ids,
                    disable_cors,
                    disable_csp,
                    follow_redirects,
                    http_version,
                    ignore_server_certificate_error,
                    initial_navigation_timeout,
                    min_failure_duration,
                    min_location_failed,
                    monitor_name,
                    monitor_options,
                    monitor_priority,
                    no_screenshot,
                    restricted_roles,
                    retry,
                    rum_settings,
                    scheduling,
                    tick_every,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestOptionsVisitor)
    }
}
