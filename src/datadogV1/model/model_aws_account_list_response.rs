// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of enabled AWS accounts.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AWSAccountListResponse {
    /// List of enabled AWS accounts.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV1::model::AWSAccount>>,
}

impl AWSAccountListResponse {
    pub fn new() -> AWSAccountListResponse {
        AWSAccountListResponse { accounts: None }
    }
}
