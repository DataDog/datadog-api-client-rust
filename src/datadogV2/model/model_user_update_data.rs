// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to update a user.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateData {
    /// Attributes of the edited user.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::UserUpdateAttributes,
    /// ID of the user.
    #[serde(rename = "id")]
    pub id: String,
    /// Users resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UsersType,
}

impl UserUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::UserUpdateAttributes,
        id: String,
        type_: crate::datadogV2::model::UsersType,
    ) -> UserUpdateData {
        UserUpdateData {
            attributes,
            id,
            type_,
        }
    }
}
