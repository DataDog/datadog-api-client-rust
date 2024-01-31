// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The expected response schema when getting a Confluent account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountResponse {
    /// An API key and API secret pair that represents a Confluent account.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::ConfluentAccountResponseData>,
}

impl ConfluentAccountResponse {
    pub fn new() -> ConfluentAccountResponse {
        ConfluentAccountResponse { data: None }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV2::model::ConfluentAccountResponseData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for ConfluentAccountResponse {
    fn default() -> Self {
        Self::new()
    }
}
