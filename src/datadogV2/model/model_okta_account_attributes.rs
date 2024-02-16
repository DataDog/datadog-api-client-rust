// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes object for an Okta account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OktaAccountAttributes {
    /// The API key of the Okta account.
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    /// The authorization method for an Okta account.
    #[serde(rename = "auth_method")]
    pub auth_method: String,
    /// The Client ID of an Okta app integration.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// The client secret of an Okta app integration.
    #[serde(rename = "client_secret")]
    pub client_secret: Option<String>,
    /// The domain of the Okta account.
    #[serde(rename = "domain")]
    pub domain: String,
    /// The name of the Okta account.
    #[serde(rename = "name")]
    pub name: String,
}

impl OktaAccountAttributes {
    pub fn new(auth_method: String, domain: String, name: String) -> OktaAccountAttributes {
        OktaAccountAttributes {
            api_key: None,
            auth_method,
            client_id: None,
            client_secret: None,
            domain,
            name,
        }
    }

    pub fn api_key(&mut self, value: String) -> &mut Self {
        self.api_key = Some(value);
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
}
