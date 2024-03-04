// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// On-demand concurrency cap response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnDemandConcurrencyCapResponse {
    /// On-demand concurrency cap.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::OnDemandConcurrencyCap>,
}

impl OnDemandConcurrencyCapResponse {
    pub fn new() -> OnDemandConcurrencyCapResponse {
        OnDemandConcurrencyCapResponse { data: None }
    }

    pub fn data(mut self, value: crate::datadogV2::model::OnDemandConcurrencyCap) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for OnDemandConcurrencyCapResponse {
    fn default() -> Self {
        Self::new()
    }
}
