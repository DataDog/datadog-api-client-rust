// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptions {
    /// For SSL test, whether or not the test should allow self signed
certificates.
    #[serde(rename = "accept_self_signed", skip_serializing_if = "Option::is_none")]
    pub accept_self_signed: bool,
    /// Allows loading insecure content for an HTTP request in an API test.
    #[serde(rename = "allow_insecure", skip_serializing_if = "Option::is_none")]
    pub allow_insecure: bool,
    /// For SSL test, whether or not the test should fail on revoked certificate in stapled OCSP.
    #[serde(rename = "checkCertificateRevocation", skip_serializing_if = "Option::is_none")]
    pub check_certificate_revocation: bool,
    /// CI/CD options for a Synthetic test.
    #[serde(rename = "ci", skip_serializing_if = "Option::is_none")]
    pub ci: SyntheticsTestCiOptions,
    /// For browser test, array with the different device IDs used to run the test.
    #[serde(rename = "device_ids", skip_serializing_if = "Option::is_none")]
    pub device_ids: Vec<SyntheticsDeviceID>,
    /// Whether or not to disable CORS mechanism.
    #[serde(rename = "disableCors", skip_serializing_if = "Option::is_none")]
    pub disable_cors: bool,
    /// Disable Content Security Policy for browser tests.
    #[serde(rename = "disableCsp", skip_serializing_if = "Option::is_none")]
    pub disable_csp: bool,
    /// For API HTTP test, whether or not the test should follow redirects.
    #[serde(rename = "follow_redirects", skip_serializing_if = "Option::is_none")]
    pub follow_redirects: bool,
    /// HTTP version to use for a Synthetic test.
    #[serde(rename = "httpVersion", skip_serializing_if = "Option::is_none")]
    pub http_version: SyntheticsTestOptionsHTTPVersion,
    /// Ignore server certificate error for browser tests.
    #[serde(rename = "ignoreServerCertificateError", skip_serializing_if = "Option::is_none")]
    pub ignore_server_certificate_error: bool,
    /// Timeout before declaring the initial step as failed (in seconds) for browser tests.
    #[serde(rename = "initialNavigationTimeout", skip_serializing_if = "Option::is_none")]
    pub initial_navigation_timeout: i64,
    /// Minimum amount of time in failure required to trigger an alert.
    #[serde(rename = "min_failure_duration", skip_serializing_if = "Option::is_none")]
    pub min_failure_duration: i64,
    /// Minimum number of locations in failure required to trigger
an alert.
    #[serde(rename = "min_location_failed", skip_serializing_if = "Option::is_none")]
    pub min_location_failed: i64,
    /// The monitor name is used for the alert title as well as for all monitor dashboard widgets and SLOs.
    #[serde(rename = "monitor_name", skip_serializing_if = "Option::is_none")]
    pub monitor_name: String,
    /// Object containing the options for a Synthetic test as a monitor
(for example, renotification).
    #[serde(rename = "monitor_options", skip_serializing_if = "Option::is_none")]
    pub monitor_options: SyntheticsTestOptionsMonitorOptions,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(rename = "monitor_priority", skip_serializing_if = "Option::is_none")]
    pub monitor_priority: i32,
    /// Prevents saving screenshots of the steps.
    #[serde(rename = "noScreenshot", skip_serializing_if = "Option::is_none")]
    pub no_screenshot: bool,
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access.
    #[serde(rename = "restricted_roles", skip_serializing_if = "Option::is_none")]
    pub restricted_roles: Vec<String>,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: SyntheticsTestOptionsRetry,
    /// The RUM data collection settings for the Synthetic browser test.
**Note:** There are 3 ways to format RUM settings:

`{ isEnabled: false }`
RUM data is not collected.

`{ isEnabled: true }`
RUM data is collected from the Synthetic test's default application.

`{ isEnabled: true, applicationId: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", clientTokenId: 12345 }`
RUM data is collected using the specified application.
    #[serde(rename = "rumSettings")]
    pub rum_settings: SyntheticsBrowserTestRumSettings,
    /// Object containing timeframes and timezone used for advanced scheduling.
    #[serde(rename = "scheduling", skip_serializing_if = "Option::is_none")]
    pub scheduling: SyntheticsTestOptionsScheduling,
    /// The frequency at which to run the Synthetic test (in seconds).
    #[serde(rename = "tick_every", skip_serializing_if = "Option::is_none")]
    pub tick_every: i64,
}

