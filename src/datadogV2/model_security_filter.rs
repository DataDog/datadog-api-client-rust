// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilter {
    /// The object describing a security filter.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SecurityFilterAttributes,
    /// The ID of the security filter.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The type of the resource. The value should always be `security_filters`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SecurityFilterType,
}

