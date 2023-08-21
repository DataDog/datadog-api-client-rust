// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullApplicationKeyAttributes {
    /// Creation date of the application key.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// The application key.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: String,
    /// The last four characters of the application key.
    #[serde(rename = "last4", skip_serializing_if = "Option::is_none")]
    pub last4: String,
    /// Name of the application key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub scopes: datadog.NullableList[String],
}

