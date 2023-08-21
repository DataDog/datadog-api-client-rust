// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsListResponse {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: String,
    /// List of ARNs configured in your Datadog account.
    #[serde(rename = "lambdas", skip_serializing_if = "Option::is_none")]
    pub lambdas: Vec<AWSLogsLambda>,
    /// Array of services IDs.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Vec<String>,
}

