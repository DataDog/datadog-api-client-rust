// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Your delegate service account response data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSDelegateAccountResponse {
    /// Datadog principal service account info.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GCPSTSDelegateAccount>,
}

impl GCPSTSDelegateAccountResponse {
    pub fn new() -> GCPSTSDelegateAccountResponse {
        GCPSTSDelegateAccountResponse { data: None }
    }

    pub fn data(mut self, value: crate::datadogV2::model::GCPSTSDelegateAccount) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for GCPSTSDelegateAccountResponse {
    fn default() -> Self {
        Self::new()
    }
}
