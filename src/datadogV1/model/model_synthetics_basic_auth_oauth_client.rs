// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to handle `oauth client` authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBasicAuthOauthClient {
    /// Access token URL to use when performing the authentication.
    #[serde(rename = "accessTokenUrl")]
    pub access_token_url: String,
    /// Audience to use when performing the authentication.
    #[serde(rename = "audience")]
    pub audience: Option<String>,
    /// Client ID to use when performing the authentication.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Client secret to use when performing the authentication.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// Resource to use when performing the authentication.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// Scope to use when performing the authentication.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// Type of token to use when performing the authentication.
    #[serde(rename = "tokenApiAuthentication")]
    pub token_api_authentication:
        crate::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication,
    /// The type of basic authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsBasicAuthOauthClientType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBasicAuthOauthClient {
    pub fn new(
        access_token_url: String,
        client_id: String,
        client_secret: String,
        token_api_authentication: crate::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication,
    ) -> SyntheticsBasicAuthOauthClient {
        SyntheticsBasicAuthOauthClient {
            access_token_url,
            audience: None,
            client_id,
            client_secret,
            resource: None,
            scope: None,
            token_api_authentication,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn audience(mut self, value: String) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn resource(mut self, value: String) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV1::model::SyntheticsBasicAuthOauthClientType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthOauthClient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthOauthClientVisitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthOauthClientVisitor {
            type Value = SyntheticsBasicAuthOauthClient;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_token_url: Option<String> = None;
                let mut audience: Option<String> = None;
                let mut client_id: Option<String> = None;
                let mut client_secret: Option<String> = None;
                let mut resource: Option<String> = None;
                let mut scope: Option<String> = None;
                let mut token_api_authentication: Option<
                    crate::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication,
                > = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBasicAuthOauthClientType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accessTokenUrl" => {
                            access_token_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audience" => {
                            if v.is_null() {
                                continue;
                            }
                            audience = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "clientId" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "clientSecret" => {
                            client_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource" => {
                            if v.is_null() {
                                continue;
                            }
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tokenApiAuthentication" => {
                            token_api_authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _token_api_authentication) = token_api_authentication {
                                match _token_api_authentication {
                                    crate::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication::UnparsedObject(_token_api_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBasicAuthOauthClientType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let access_token_url =
                    access_token_url.ok_or_else(|| M::Error::missing_field("access_token_url"))?;
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let client_secret =
                    client_secret.ok_or_else(|| M::Error::missing_field("client_secret"))?;
                let token_api_authentication = token_api_authentication
                    .ok_or_else(|| M::Error::missing_field("token_api_authentication"))?;

                let content = SyntheticsBasicAuthOauthClient {
                    access_token_url,
                    audience,
                    client_id,
                    client_secret,
                    resource,
                    scope,
                    token_api_authentication,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthOauthClientVisitor)
    }
}
