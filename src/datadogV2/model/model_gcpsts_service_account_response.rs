// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The account creation response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountResponse {
    /// Info on your service account.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GCPSTSServiceAccount>,
}

impl GCPSTSServiceAccountResponse {
    pub fn new() -> GCPSTSServiceAccountResponse {
        GCPSTSServiceAccountResponse { data: None }
    }

    pub fn with_data(&mut self, value: crate::datadogV2::model::GCPSTSServiceAccount) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for GCPSTSServiceAccountResponse {
    fn default() -> Self {
        Self::new()
    }
}
