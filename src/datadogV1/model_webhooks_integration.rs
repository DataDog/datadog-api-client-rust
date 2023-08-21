// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksIntegration {
    /// If `null`, uses no header.
If given a JSON payload, these will be headers attached to your webhook.
    #[serde(rename = "custom_headers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub custom_headers: Option<String>,
    /// Encoding type. Can be given either `json` or `form`.
    #[serde(rename = "encode_as", skip_serializing_if = "Option::is_none")]
    pub encode_as: WebhooksIntegrationEncoding,
    /// The name of the webhook. It corresponds with `<WEBHOOK_NAME>`.
Learn more on how to use it in
[monitor notifications](https://docs.datadoghq.com/monitors/notify).
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// If `null`, uses the default payload.
If given a JSON payload, the webhook returns the payload
specified by the given payload.
[Webhooks variable usage](https://docs.datadoghq.com/integrations/webhooks/#usage).
    #[serde(rename = "payload", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub payload: Option<String>,
    /// URL of the webhook.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

