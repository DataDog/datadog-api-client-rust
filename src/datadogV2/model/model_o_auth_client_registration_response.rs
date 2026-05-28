// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response payload for a successful OAuth2 dynamic client registration as defined by RFC 7591.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OAuthClientRegistrationResponse {
    /// Unique identifier assigned to the registered client.
    #[serde(rename = "client_id")]
    pub client_id: uuid::Uuid,
    /// Human-readable name of the client.
    #[serde(rename = "client_name")]
    pub client_name: String,
    /// OAuth 2.0 grant types registered for the client.
    #[serde(rename = "grant_types")]
    pub grant_types: Vec<crate::datadogV2::model::OAuthClientRegistrationGrantType>,
    /// Redirection URIs registered for the client.
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<String>,
    /// OAuth 2.0 response types registered for the client.
    #[serde(rename = "response_types")]
    pub response_types: Vec<crate::datadogV2::model::OAuthClientRegistrationResponseType>,
    /// Authentication method registered for the token endpoint. Always `none`.
    #[serde(rename = "token_endpoint_auth_method")]
    pub token_endpoint_auth_method: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OAuthClientRegistrationResponse {
    pub fn new(
        client_id: uuid::Uuid,
        client_name: String,
        grant_types: Vec<crate::datadogV2::model::OAuthClientRegistrationGrantType>,
        redirect_uris: Vec<String>,
        response_types: Vec<crate::datadogV2::model::OAuthClientRegistrationResponseType>,
        token_endpoint_auth_method: String,
    ) -> OAuthClientRegistrationResponse {
        OAuthClientRegistrationResponse {
            client_id,
            client_name,
            grant_types,
            redirect_uris,
            response_types,
            token_endpoint_auth_method,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for OAuthClientRegistrationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OAuthClientRegistrationResponseVisitor;
        impl<'a> Visitor<'a> for OAuthClientRegistrationResponseVisitor {
            type Value = OAuthClientRegistrationResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_id: Option<uuid::Uuid> = None;
                let mut client_name: Option<String> = None;
                let mut grant_types: Option<
                    Vec<crate::datadogV2::model::OAuthClientRegistrationGrantType>,
                > = None;
                let mut redirect_uris: Option<Vec<String>> = None;
                let mut response_types: Option<
                    Vec<crate::datadogV2::model::OAuthClientRegistrationResponseType>,
                > = None;
                let mut token_endpoint_auth_method: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_name" => {
                            client_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grant_types" => {
                            grant_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_uris" => {
                            redirect_uris =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_types" => {
                            response_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token_endpoint_auth_method" => {
                            token_endpoint_auth_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let client_name =
                    client_name.ok_or_else(|| M::Error::missing_field("client_name"))?;
                let grant_types =
                    grant_types.ok_or_else(|| M::Error::missing_field("grant_types"))?;
                let redirect_uris =
                    redirect_uris.ok_or_else(|| M::Error::missing_field("redirect_uris"))?;
                let response_types =
                    response_types.ok_or_else(|| M::Error::missing_field("response_types"))?;
                let token_endpoint_auth_method = token_endpoint_auth_method
                    .ok_or_else(|| M::Error::missing_field("token_endpoint_auth_method"))?;

                let content = OAuthClientRegistrationResponse {
                    client_id,
                    client_name,
                    grant_types,
                    redirect_uris,
                    response_types,
                    token_endpoint_auth_method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OAuthClientRegistrationResponseVisitor)
    }
}
