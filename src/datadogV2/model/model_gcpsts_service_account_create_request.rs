// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data on your newly generated service account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountCreateRequest {
    /// Additional metadata on your generated service account.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GCPSTSServiceAccountData>,
}

impl GCPSTSServiceAccountCreateRequest {
    pub fn new() -> GCPSTSServiceAccountCreateRequest {
        GCPSTSServiceAccountCreateRequest { data: None }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV2::model::GCPSTSServiceAccountData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for GCPSTSServiceAccountCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}
