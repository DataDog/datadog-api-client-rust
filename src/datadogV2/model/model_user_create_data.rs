// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to create a user.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateData {
    /// Attributes of the created user.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::UserCreateAttributes,
    /// Relationships of the user object.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::UserRelationships>,
    /// Users resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UsersType,
}

impl UserCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::UserCreateAttributes,
        type_: crate::datadogV2::model::UsersType,
    ) -> UserCreateData {
        UserCreateData {
            attributes,
            relationships: None,
            type_,
        }
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::UserRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}
