// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestRequestProxy {
    /// Headers to include when performing the test.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: map[string]String,
    /// URL of the proxy to perform the test.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

