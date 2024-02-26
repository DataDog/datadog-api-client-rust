// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes associated with the account creation request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountCreateRequestAttributes {
    /// The API key associated with your Confluent account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The API secret associated with your Confluent account.
    #[serde(rename = "api_secret")]
    pub api_secret: String,
    /// A list of Confluent resources associated with the Confluent account.
    #[serde(rename = "resources")]
    pub resources: Option<Vec<crate::datadogV2::model::ConfluentAccountResourceAttributes>>,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl ConfluentAccountCreateRequestAttributes {
    pub fn new(api_key: String, api_secret: String) -> ConfluentAccountCreateRequestAttributes {
        ConfluentAccountCreateRequestAttributes {
            api_key,
            api_secret,
            resources: None,
            tags: None,
        }
    }

    pub fn resources(
        &mut self,
        value: Vec<crate::datadogV2::model::ConfluentAccountResourceAttributes>,
    ) -> &mut Self {
        self.resources = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
