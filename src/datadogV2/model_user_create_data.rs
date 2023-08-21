// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateData {
    /// Attributes of the created user.
    #[serde(rename = "attributes")]
    pub attributes: UserCreateAttributes,
    /// Relationships of the user object.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: UserRelationships,
    /// Users resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: UsersType,
}

