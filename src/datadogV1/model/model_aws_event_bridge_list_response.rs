// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object describing the EventBridge configuration for multiple accounts.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeListResponse {
    /// List of accounts with their event sources.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV1::model::AWSEventBridgeAccountConfiguration>>,
    /// True if the EventBridge sub-integration is enabled for your organization.
    #[serde(rename = "isInstalled")]
    pub is_installed: Option<bool>,
}

impl AWSEventBridgeListResponse {
    pub fn new() -> AWSEventBridgeListResponse {
        AWSEventBridgeListResponse {
            accounts: None,
            is_installed: None,
        }
    }

    pub fn with_accounts(
        &mut self,
        value: Vec<crate::datadogV1::model::AWSEventBridgeAccountConfiguration>,
    ) -> &mut Self {
        self.accounts = Some(value);
        self
    }

    pub fn with_is_installed(&mut self, value: bool) -> &mut Self {
        self.is_installed = Some(value);
        self
    }
}
impl Default for AWSEventBridgeListResponse {
    fn default() -> Self {
        Self::new()
    }
}
