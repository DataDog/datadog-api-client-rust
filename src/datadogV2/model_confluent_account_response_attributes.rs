// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountResponseAttributes {
    /// The API key associated with your Confluent account.
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: String,
    /// A list of Confluent resources associated with the Confluent account.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Vec<ConfluentResourceResponseAttributes>,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

