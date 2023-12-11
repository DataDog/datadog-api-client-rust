// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of a Confluent account.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountResponseAttributes {
    /// The API key associated with your Confluent account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// A list of Confluent resources associated with the Confluent account.
    #[serde(rename = "resources")]
    pub resources: Option<Vec<crate::datadogV2::model::ConfluentResourceResponseAttributes>>,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl ConfluentAccountResponseAttributes {
    pub fn new(api_key: String) -> ConfluentAccountResponseAttributes {
        ConfluentAccountResponseAttributes {
            api_key,
            resources: None,
            tags: None,
        }
    }
}
