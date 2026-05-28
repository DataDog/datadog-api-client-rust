// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request payload for OAuth2 dynamic client registration as defined by RFC 7591.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OAuthClientRegistrationRequest {
    /// Human-readable name of the client. Control characters are rejected.
    #[serde(rename = "client_name")]
    pub client_name: String,
    /// URL of the home page of the client.
    #[serde(rename = "client_uri")]
    pub client_uri: Option<String>,
    /// OAuth 2.0 grant types the client may use.
    /// Defaults to `authorization_code` and `refresh_token` when omitted.
    #[serde(rename = "grant_types")]
    pub grant_types: Option<Vec<crate::datadogV2::model::OAuthClientRegistrationGrantType>>,
    /// URL referencing the client's JSON Web Key Set.
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: Option<String>,
    /// URL referencing a logo for the client.
    #[serde(rename = "logo_uri")]
    pub logo_uri: Option<String>,
    /// URL pointing to the client's privacy policy.
    #[serde(rename = "policy_uri")]
    pub policy_uri: Option<String>,
    /// Array of redirection URI strings used by the client in redirect-based flows.
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<String>,
    /// OAuth 2.0 response types the client may use. Only `code` is supported.
    #[serde(rename = "response_types")]
    pub response_types: Option<Vec<crate::datadogV2::model::OAuthClientRegistrationResponseType>>,
    /// Space-separated list of scope values the client may request.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// Requested authentication method for the token endpoint. Only `none` is supported.
    #[serde(rename = "token_endpoint_auth_method")]
    pub token_endpoint_auth_method: Option<String>,
    /// URL pointing to the client's terms of service.
    #[serde(rename = "tos_uri")]
    pub tos_uri: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OAuthClientRegistrationRequest {
    pub fn new(client_name: String, redirect_uris: Vec<String>) -> OAuthClientRegistrationRequest {
        OAuthClientRegistrationRequest {
            client_name,
            client_uri: None,
            grant_types: None,
            jwks_uri: None,
            logo_uri: None,
            policy_uri: None,
            redirect_uris,
            response_types: None,
            scope: None,
            token_endpoint_auth_method: None,
            tos_uri: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn client_uri(mut self, value: String) -> Self {
        self.client_uri = Some(value);
        self
    }

    pub fn grant_types(
        mut self,
        value: Vec<crate::datadogV2::model::OAuthClientRegistrationGrantType>,
    ) -> Self {
        self.grant_types = Some(value);
        self
    }

    pub fn jwks_uri(mut self, value: String) -> Self {
        self.jwks_uri = Some(value);
        self
    }

    pub fn logo_uri(mut self, value: String) -> Self {
        self.logo_uri = Some(value);
        self
    }

    pub fn policy_uri(mut self, value: String) -> Self {
        self.policy_uri = Some(value);
        self
    }

    pub fn response_types(
        mut self,
        value: Vec<crate::datadogV2::model::OAuthClientRegistrationResponseType>,
    ) -> Self {
        self.response_types = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn token_endpoint_auth_method(mut self, value: String) -> Self {
        self.token_endpoint_auth_method = Some(value);
        self
    }

    pub fn tos_uri(mut self, value: String) -> Self {
        self.tos_uri = Some(value);
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

impl<'de> Deserialize<'de> for OAuthClientRegistrationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OAuthClientRegistrationRequestVisitor;
        impl<'a> Visitor<'a> for OAuthClientRegistrationRequestVisitor {
            type Value = OAuthClientRegistrationRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_name: Option<String> = None;
                let mut client_uri: Option<String> = None;
                let mut grant_types: Option<
                    Vec<crate::datadogV2::model::OAuthClientRegistrationGrantType>,
                > = None;
                let mut jwks_uri: Option<String> = None;
                let mut logo_uri: Option<String> = None;
                let mut policy_uri: Option<String> = None;
                let mut redirect_uris: Option<Vec<String>> = None;
                let mut response_types: Option<
                    Vec<crate::datadogV2::model::OAuthClientRegistrationResponseType>,
                > = None;
                let mut scope: Option<String> = None;
                let mut token_endpoint_auth_method: Option<String> = None;
                let mut tos_uri: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_name" => {
                            client_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            client_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grant_types" => {
                            if v.is_null() {
                                continue;
                            }
                            grant_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jwks_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            jwks_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logo_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            logo_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_uris" => {
                            redirect_uris =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_types" => {
                            if v.is_null() {
                                continue;
                            }
                            response_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token_endpoint_auth_method" => {
                            if v.is_null() {
                                continue;
                            }
                            token_endpoint_auth_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tos_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            tos_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let client_name =
                    client_name.ok_or_else(|| M::Error::missing_field("client_name"))?;
                let redirect_uris =
                    redirect_uris.ok_or_else(|| M::Error::missing_field("redirect_uris"))?;

                let content = OAuthClientRegistrationRequest {
                    client_name,
                    client_uri,
                    grant_types,
                    jwks_uri,
                    logo_uri,
                    policy_uri,
                    redirect_uris,
                    response_types,
                    scope,
                    token_endpoint_auth_method,
                    tos_uri,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OAuthClientRegistrationRequestVisitor)
    }
}
