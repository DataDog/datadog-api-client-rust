// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of AWS related accounts.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSRelatedAccountsResponse {
    /// An AWS related account.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::AWSRelatedAccount>>,
}

impl AWSRelatedAccountsResponse {
    pub fn new() -> AWSRelatedAccountsResponse {
        AWSRelatedAccountsResponse { data: None }
    }

    pub fn with_data(
        &mut self,
        value: Vec<crate::datadogV2::model::AWSRelatedAccount>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for AWSRelatedAccountsResponse {
    fn default() -> Self {
        Self::new()
    }
}
