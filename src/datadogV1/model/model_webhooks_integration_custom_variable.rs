// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Custom variable for Webhook integration.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhooksIntegrationCustomVariable {
    /// Make custom variable is secret or not.
    /// If the custom variable is secret, the value is not returned in the response payload.
    #[serde(rename = "is_secret")]
    pub is_secret: bool,
    /// The name of the variable. It corresponds with `<CUSTOM_VARIABLE_NAME>`.
    #[serde(rename = "name")]
    pub name: String,
    /// Value of the custom variable.
    #[serde(rename = "value")]
    pub value: String,
}

impl WebhooksIntegrationCustomVariable {
    pub fn new(is_secret: bool, name: String, value: String) -> WebhooksIntegrationCustomVariable {
        WebhooksIntegrationCustomVariable {
            is_secret,
            name,
            value,
        }
    }
}
