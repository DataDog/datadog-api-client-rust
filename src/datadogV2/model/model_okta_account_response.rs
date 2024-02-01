// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object for an Okta account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OktaAccountResponse {
    /// Schema for an Okta account.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::OktaAccount>,
}

impl OktaAccountResponse {
    pub fn new() -> OktaAccountResponse {
        OktaAccountResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::OktaAccount) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for OktaAccountResponse {
    fn default() -> Self {
        Self::new()
    }
}
