// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The expected response schema when getting a Fastly account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountResponse {
    /// Data object of a Fastly account.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::FastlyAccountResponseData>>,
}

impl FastlyAccountResponse {
    pub fn new() -> FastlyAccountResponse {
        FastlyAccountResponse { data: None }
    }
}
impl Default for FastlyAccountResponse {
    fn default() -> Self {
        Self::new()
    }
}
