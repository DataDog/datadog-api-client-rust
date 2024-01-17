// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update request of a Webhooks integration object.
///
/// *All properties are optional.*
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksIntegrationUpdateRequest {
    /// If `null`, uses no header.
    /// If given a JSON payload, these will be headers attached to your webhook.
    #[serde(rename = "custom_headers")]
    pub custom_headers: Option<String>,
    /// Encoding type. Can be given either `json` or `form`.
    #[serde(rename = "encode_as")]
    pub encode_as: Option<crate::datadogV1::model::WebhooksIntegrationEncoding>,
    /// The name of the webhook. It corresponds with `<WEBHOOK_NAME>`.
    /// Learn more on how to use it in
    /// [monitor notifications](https://docs.datadoghq.com/monitors/notify).
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// If `null`, uses the default payload.
    /// If given a JSON payload, the webhook returns the payload
    /// specified by the given payload.
    /// [Webhooks variable usage](https://docs.datadoghq.com/integrations/webhooks/#usage).
    #[serde(
        rename = "payload",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub payload: Option<Option<String>>,
    /// URL of the webhook.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl WebhooksIntegrationUpdateRequest {
    pub fn new() -> WebhooksIntegrationUpdateRequest {
        WebhooksIntegrationUpdateRequest {
            custom_headers: None,
            encode_as: None,
            name: None,
            payload: None,
            url: None,
        }
    }
}
impl Default for WebhooksIntegrationUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
