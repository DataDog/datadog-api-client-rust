// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// OAuth2 client credentials attributes returned by the API. The `client_secret` is never echoed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WebhooksOAuth2ClientCredentialsResponseAttributes {
    /// URL of the OAuth2 access token endpoint.
    #[serde(rename = "access_token_url")]
    pub access_token_url: Option<String>,
    /// The intended audience for the OAuth2 access token.
    #[serde(
        rename = "audience",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub audience: Option<Option<String>>,
    /// The OAuth2 client ID issued by the authorization server.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// Human-readable name for this auth method.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Authentication protocol used by the auth method.
    #[serde(rename = "protocol")]
    pub protocol: Option<crate::datadogV2::model::WebhooksAuthMethodProtocol>,
    /// Space-separated list of OAuth2 scopes to request.
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option")]
    pub scope: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebhooksOAuth2ClientCredentialsResponseAttributes {
    pub fn new() -> WebhooksOAuth2ClientCredentialsResponseAttributes {
        WebhooksOAuth2ClientCredentialsResponseAttributes {
            access_token_url: None,
            audience: None,
            client_id: None,
            name: None,
            protocol: None,
            scope: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn access_token_url(mut self, value: String) -> Self {
        self.access_token_url = Some(value);
        self
    }

    pub fn audience(mut self, value: Option<String>) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn protocol(mut self, value: crate::datadogV2::model::WebhooksAuthMethodProtocol) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn scope(mut self, value: Option<String>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for WebhooksOAuth2ClientCredentialsResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WebhooksOAuth2ClientCredentialsResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhooksOAuth2ClientCredentialsResponseAttributesVisitor;
        impl<'a> Visitor<'a> for WebhooksOAuth2ClientCredentialsResponseAttributesVisitor {
            type Value = WebhooksOAuth2ClientCredentialsResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_token_url: Option<String> = None;
                let mut audience: Option<Option<String>> = None;
                let mut client_id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut protocol: Option<crate::datadogV2::model::WebhooksAuthMethodProtocol> =
                    None;
                let mut scope: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_token_url" => {
                            if v.is_null() {
                                continue;
                            }
                            access_token_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audience" => {
                            audience = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protocol" => {
                            if v.is_null() {
                                continue;
                            }
                            protocol = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _protocol) = protocol {
                                match _protocol {
                                    crate::datadogV2::model::WebhooksAuthMethodProtocol::UnparsedObject(_protocol) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = WebhooksOAuth2ClientCredentialsResponseAttributes {
                    access_token_url,
                    audience,
                    client_id,
                    name,
                    protocol,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebhooksOAuth2ClientCredentialsResponseAttributesVisitor)
    }
}
