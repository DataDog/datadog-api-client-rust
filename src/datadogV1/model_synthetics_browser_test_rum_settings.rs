// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestRumSettings {
    /// RUM application ID used to collect RUM data for the browser test.
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: String,
    /// RUM application API key ID used to collect RUM data for the browser test.
    #[serde(rename = "clientTokenId", skip_serializing_if = "Option::is_none")]
    pub client_token_id: i64,
    /// Determines whether RUM data is collected during test runs.
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
}

