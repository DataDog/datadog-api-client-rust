// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// User object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Attributes of user object returned by the API.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::UserAttributes>,
    /// ID of the user.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the user object returned by the API.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::UserResponseRelationships>,
    /// Users resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::UsersType>,
}

impl User {
    pub fn new() -> User {
        User {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::UserAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::UserResponseRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::UsersType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}
