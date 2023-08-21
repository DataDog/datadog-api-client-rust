// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingCreateAttributes {
    /// Key portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_key", skip_serializing_if = "Option::is_none")]
    pub attribute_key: String,
    /// Value portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_value", skip_serializing_if = "Option::is_none")]
    pub attribute_value: String,
}

