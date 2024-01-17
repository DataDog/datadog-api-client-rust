// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of AWS accounts to delete.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSAccountDeleteRequest {
    /// Your AWS access key ID. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "access_key_id")]
    pub access_key_id: Option<String>,
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Your Datadog role delegation name.
    #[serde(rename = "role_name")]
    pub role_name: Option<String>,
}

impl AWSAccountDeleteRequest {
    pub fn new() -> AWSAccountDeleteRequest {
        AWSAccountDeleteRequest {
            access_key_id: None,
            account_id: None,
            role_name: None,
        }
    }
}
impl Default for AWSAccountDeleteRequest {
    fn default() -> Self {
        Self::new()
    }
}
