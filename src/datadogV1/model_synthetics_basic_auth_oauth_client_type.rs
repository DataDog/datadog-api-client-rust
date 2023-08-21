// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsBasicAuthOauthClientType {
    #[serde(rename = "oauth-client")]
	OAUTH_CLIENT,
}

impl ToString for SyntheticsBasicAuthOauthClientType {
    fn to_string(&self) -> String {
        match self {
            Self::OAUTH_CLIENT => String::from("oauth-client"),
        }
    }
}

impl Default for SyntheticsBasicAuthOauthClientType {
    fn default() -> SyntheticsBasicAuthOauthClientType {
        Self::OAUTH_CLIENT
    }
}
