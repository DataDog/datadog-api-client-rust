// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKey {
    /// Hash of an application key.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: String,
    /// Name of an application key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Owner of an application key.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: String,
}

