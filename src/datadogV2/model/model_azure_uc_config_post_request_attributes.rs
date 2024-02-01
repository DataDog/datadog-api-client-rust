// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for Azure config Post Request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureUCConfigPostRequestAttributes {
    /// The tenant ID of the azure account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Bill config.
    #[serde(rename = "actual_bill_config")]
    pub actual_bill_config: crate::datadogV2::model::BillConfig,
    /// Bill config.
    #[serde(rename = "amortized_bill_config")]
    pub amortized_bill_config: crate::datadogV2::model::BillConfig,
    /// The client ID of the azure account.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Whether or not the Cloud Cost Management account is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The scope of your observed subscription.
    #[serde(rename = "scope")]
    pub scope: String,
}

impl AzureUCConfigPostRequestAttributes {
    pub fn new(
        account_id: String,
        actual_bill_config: crate::datadogV2::model::BillConfig,
        amortized_bill_config: crate::datadogV2::model::BillConfig,
        client_id: String,
        scope: String,
    ) -> AzureUCConfigPostRequestAttributes {
        AzureUCConfigPostRequestAttributes {
            account_id,
            actual_bill_config,
            amortized_bill_config,
            client_id,
            is_enabled: None,
            scope,
        }
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }
}
