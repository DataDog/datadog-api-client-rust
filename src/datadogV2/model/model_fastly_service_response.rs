// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The expected response schema when getting a Fastly service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyServiceResponse {
    /// Data object for Fastly service requests.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::FastlyServiceData>,
}

impl FastlyServiceResponse {
    pub fn new() -> FastlyServiceResponse {
        FastlyServiceResponse { data: None }
    }

    pub fn with_data(&mut self, value: crate::datadogV2::model::FastlyServiceData) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for FastlyServiceResponse {
    fn default() -> Self {
        Self::new()
    }
}
