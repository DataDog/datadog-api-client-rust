// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to handle `oauth client` authentication when performing the test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }
}
