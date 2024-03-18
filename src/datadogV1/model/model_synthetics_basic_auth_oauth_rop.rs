// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to handle `oauth rop` authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBasicAuthOauthROP {
    /// Access token URL to use when performing the authentication.
    #[serde(rename = "accessTokenUrl")]
    pub access_token_url: String,
    /// Audience to use when performing the authentication.
    #[serde(rename = "audience")]
    pub audience: Option<String>,
    /// Client ID to use when performing the authentication.
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    /// Client secret to use when performing the authentication.
    #[serde(rename = "clientSecret")]
    pub client_secret: Option<String>,
    /// Password to use when performing the authentication.
    #[serde(rename = "password")]
    pub password: String,
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
    pub type_: Option<crate::datadogV1::model::SyntheticsBasicAuthOauthROPType>,
    /// Username to use when performing the authentication.
    #[serde(rename = "username")]
    pub username: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBasicAuthOauthROP {
    pub fn new(
        access_token_url: String,
        password: String,
        token_api_authentication: crate::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication,
        username: String,
    ) -> SyntheticsBasicAuthOauthROP {
        SyntheticsBasicAuthOauthROP {
            access_token_url,
            audience: None,
            client_id: None,
            client_secret: None,
            password,
            resource: None,
            scope: None,
            token_api_authentication,
            type_: None,
            username,
            _unparsed: false,
        }
    }

    pub fn audience(mut self, value: String) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn client_secret(mut self, value: String) -> Self {
        self.client_secret = Some(value);
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
        value: crate::datadogV1::model::SyntheticsBasicAuthOauthROPType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthOauthROP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthOauthROPVisitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthOauthROPVisitor {
            type Value = SyntheticsBasicAuthOauthROP;

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
                let mut password: Option<String> = None;
                let mut resource: Option<String> = None;
                let mut scope: Option<String> = None;
                let mut token_api_authentication: Option<
                    crate::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication,
                > = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBasicAuthOauthROPType> =
                    None;
                let mut username: Option<String> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "clientSecret" => {
                            if v.is_null() {
                                continue;
                            }
                            client_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "password" => {
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::SyntheticsBasicAuthOauthROPType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "username" => {
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let access_token_url =
                    access_token_url.ok_or_else(|| M::Error::missing_field("access_token_url"))?;
                let password = password.ok_or_else(|| M::Error::missing_field("password"))?;
                let token_api_authentication = token_api_authentication
                    .ok_or_else(|| M::Error::missing_field("token_api_authentication"))?;
                let username = username.ok_or_else(|| M::Error::missing_field("username"))?;

                let content = SyntheticsBasicAuthOauthROP {
                    access_token_url,
                    audience,
                    client_id,
                    client_secret,
                    password,
                    resource,
                    scope,
                    token_api_authentication,
                    type_,
                    username,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthOauthROPVisitor)
    }
}
