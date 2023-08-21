// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCITest {
    /// Disable certificate checks in API tests.
    #[serde(rename = "allowInsecureCertificates", skip_serializing_if = "Option::is_none")]
    pub allow_insecure_certificates: bool,
    /// Object to handle basic authentication when performing the test.
    #[serde(rename = "basicAuth", skip_serializing_if = "Option::is_none")]
    pub basic_auth: SyntheticsBasicAuth,
    /// Body to include in the test.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: String,
    /// Type of the data sent in a Synthetic API test.
    #[serde(rename = "bodyType", skip_serializing_if = "Option::is_none")]
    pub body_type: String,
    /// Cookies for the request.
    #[serde(rename = "cookies", skip_serializing_if = "Option::is_none")]
    pub cookies: String,
    /// For browser test, array with the different device IDs used to run the test.
    #[serde(rename = "deviceIds", skip_serializing_if = "Option::is_none")]
    pub device_ids: Vec<SyntheticsDeviceID>,
    /// For API HTTP test, whether or not the test should follow redirects.
    #[serde(rename = "followRedirects", skip_serializing_if = "Option::is_none")]
    pub follow_redirects: bool,
    /// Headers to include when performing the test.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: map[string]String,
    /// Array of locations used to run the test.
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Vec<String>,
    /// Metadata for the Synthetic tests run.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: SyntheticsCIBatchMetadata,
    /// The public ID of the Synthetic test to trigger.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: SyntheticsTestOptionsRetry,
    /// Starting URL for the browser test.
    #[serde(rename = "startUrl", skip_serializing_if = "Option::is_none")]
    pub start_url: String,
    /// Variables to replace in the test.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: map[string]String,
}

