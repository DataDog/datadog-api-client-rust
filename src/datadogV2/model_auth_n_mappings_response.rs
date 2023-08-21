// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingsResponse {
    /// Array of returned AuthN Mappings.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<AuthNMapping>,
    /// Included data in the AuthN Mapping response.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Vec<AuthNMappingIncluded>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: ResponseMetaAttributes,
}

