// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterCreateData {
    /// Object containing the attributes of the security filter to be created.
    #[serde(rename = "attributes")]
    pub attributes: SecurityFilterCreateAttributes,
    /// The type of the resource. The value should always be `security_filters`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SecurityFilterType,
}

