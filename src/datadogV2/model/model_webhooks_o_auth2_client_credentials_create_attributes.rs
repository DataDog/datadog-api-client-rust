// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// OAuth2 client credentials attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WebhooksOAuth2ClientCredentialsCreateAttributes {
    /// URL of the OAuth2 access token endpoint.
    #[serde(rename = "access_token_url")]
    pub access_token_url: String,
    /// The intended audience for the OAuth2 access token.
    #[serde(
        rename = "audience",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub audience: Option<Option<String>>,
    /// The OAuth2 client ID issued by the authorization server.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The OAuth2 client secret issued by the authorization server.
    /// Write-only; never returned by the API.
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    /// Human-readable name for this auth method. Must be unique within your organization.
    #[serde(rename = "name")]
    pub name: String,
    /// Space-separated list of OAuth2 scopes to request.
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option")]
    pub scope: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WebhooksOAuth2ClientCredentialsCreateAttributes {
    pub fn new(
        access_token_url: String,
        client_id: String,
        client_secret: String,
        name: String,
    ) -> WebhooksOAuth2ClientCredentialsCreateAttributes {
        WebhooksOAuth2ClientCredentialsCreateAttributes {
            access_token_url,
            audience: None,
            client_id,
            client_secret,
            name,
            scope: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audience(mut self, value: Option<String>) -> Self {
        self.audience = Some(value);
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

impl<'de> Deserialize<'de> for WebhooksOAuth2ClientCredentialsCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WebhooksOAuth2ClientCredentialsCreateAttributesVisitor;
        impl<'a> Visitor<'a> for WebhooksOAuth2ClientCredentialsCreateAttributesVisitor {
            type Value = WebhooksOAuth2ClientCredentialsCreateAttributes;

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
                let mut client_secret: Option<String> = None;
                let mut name: Option<String> = None;
                let mut scope: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_token_url" => {
                            access_token_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audience" => {
                            audience = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_secret" => {
                            client_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let access_token_url =
                    access_token_url.ok_or_else(|| M::Error::missing_field("access_token_url"))?;
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let client_secret =
                    client_secret.ok_or_else(|| M::Error::missing_field("client_secret"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = WebhooksOAuth2ClientCredentialsCreateAttributes {
                    access_token_url,
                    audience,
                    client_id,
                    client_secret,
                    name,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WebhooksOAuth2ClientCredentialsCreateAttributesVisitor)
    }
}
