// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthOauthROP {
    /// Access token URL to use when performing the authentication.
    #[serde(rename = "accessTokenUrl", skip_serializing_if = "Option::is_none")]
    pub access_token_url: String,
    /// Audience to use when performing the authentication.
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: String,
    /// Client ID to use when performing the authentication.
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: String,
    /// Client secret to use when performing the authentication.
    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    pub client_secret: String,
    /// Password to use when performing the authentication.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: String,
    /// Resource to use when performing the authentication.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: String,
    /// Scope to use when performing the authentication.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
    /// Type of token to use when performing the authentication.
    #[serde(rename = "tokenApiAuthentication", skip_serializing_if = "Option::is_none")]
    pub token_api_authentication: SyntheticsBasicAuthOauthTokenApiAuthentication,
    /// The type of basic authentication to use when performing the test.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsBasicAuthOauthROPType,
    /// Username to use when performing the authentication.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: String,
}

