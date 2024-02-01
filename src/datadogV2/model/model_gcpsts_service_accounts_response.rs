// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing all your STS enabled accounts.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountsResponse {
    /// Array of GCP STS enabled service accounts.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::GCPSTSServiceAccount>>,
}

impl GCPSTSServiceAccountsResponse {
    pub fn new() -> GCPSTSServiceAccountsResponse {
        GCPSTSServiceAccountsResponse { data: None }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::GCPSTSServiceAccount>) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for GCPSTSServiceAccountsResponse {
    fn default() -> Self {
        Self::new()
    }
}
