// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service account info.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountUpdateRequest {
    /// Data on your service account.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GCPSTSServiceAccountUpdateRequestData>,
}

impl GCPSTSServiceAccountUpdateRequest {
    pub fn new() -> GCPSTSServiceAccountUpdateRequest {
        GCPSTSServiceAccountUpdateRequest { data: None }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV2::model::GCPSTSServiceAccountUpdateRequestData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for GCPSTSServiceAccountUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
