// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update request of a Webhooks integration object.
///
/// *All properties are optional.*
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    /// [monitor notifications](<https://docs.datadoghq.com/monitors/notify>).
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// If `null`, uses the default payload.
    /// If given a JSON payload, the webhook returns the payload
    /// specified by the given payload.
    /// [Webhooks variable usage](<https://docs.datadoghq.com/integrations/webhooks/#usage>).
    #[serde(
        rename = "payload",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub payload: Option<Option<String>>,
    /// URL of the webhook.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebhooksIntegrationUpdateRequest {
    pub fn new() -> WebhooksIntegrationUpdateRequest {
        WebhooksIntegrationUpdateRequest {
            custom_headers: None,
            encode_as: None,
            name: None,
            payload: None,
            url: None,
            _unparsed: false,
        }
    }

    pub fn custom_headers(mut self, value: String) -> Self {
        self.custom_headers = Some(value);
        self
    }

    pub fn encode_as(
        mut self,
        value: crate::datadogV1::model::WebhooksIntegrationEncoding,
    ) -> Self {
        self.encode_as = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn payload(mut self, value: Option<String>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl Default for WebhooksIntegrationUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WebhooksIntegrationUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhooksIntegrationUpdateRequestVisitor;
        impl<'a> Visitor<'a> for WebhooksIntegrationUpdateRequestVisitor {
            type Value = WebhooksIntegrationUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_headers: Option<String> = None;
                let mut encode_as: Option<crate::datadogV1::model::WebhooksIntegrationEncoding> =
                    None;
                let mut name: Option<String> = None;
                let mut payload: Option<Option<String>> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_headers" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_headers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "encode_as" => {
                            if v.is_null() {
                                continue;
                            }
                            encode_as = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encode_as) = encode_as {
                                match _encode_as {
                                    crate::datadogV1::model::WebhooksIntegrationEncoding::UnparsedObject(_encode_as) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "payload" => {
                            payload = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WebhooksIntegrationUpdateRequest {
                    custom_headers,
                    encode_as,
                    name,
                    payload,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebhooksIntegrationUpdateRequestVisitor)
    }
}
