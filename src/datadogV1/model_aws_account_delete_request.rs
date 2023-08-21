// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSAccountDeleteRequest {
    /// Your AWS access key ID. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "access_key_id", skip_serializing_if = "Option::is_none")]
    pub access_key_id: String,
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: String,
    /// Your Datadog role delegation name.
    #[serde(rename = "role_name", skip_serializing_if = "Option::is_none")]
    pub role_name: String,
}

