// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Confluent account returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountsResponse {
    /// The Confluent account.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ConfluentAccountResponseData>>,
}

impl ConfluentAccountsResponse {
    pub fn new() -> ConfluentAccountsResponse {
        ConfluentAccountsResponse { data: None }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::ConfluentAccountResponseData>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for ConfluentAccountsResponse {
    fn default() -> Self {
        Self::new()
    }
}
