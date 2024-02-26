// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to handle `oauth rop` authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    pub fn audience(&mut self, value: String) -> &mut Self {
        self.audience = Some(value);
        self
    }

    pub fn client_id(&mut self, value: String) -> &mut Self {
        self.client_id = Some(value);
        self
    }

    pub fn client_secret(&mut self, value: String) -> &mut Self {
        self.client_secret = Some(value);
        self
    }

    pub fn resource(&mut self, value: String) -> &mut Self {
        self.resource = Some(value);
        self
    }

    pub fn scope(&mut self, value: String) -> &mut Self {
        self.scope = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV1::model::SyntheticsBasicAuthOauthROPType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
