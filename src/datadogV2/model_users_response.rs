// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersResponse {
    /// Array of returned users.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<User>,
    /// Array of objects related to the users.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Vec<UserResponseIncludedItem>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: ResponseMetaAttributes,
}

