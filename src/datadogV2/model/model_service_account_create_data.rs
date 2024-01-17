// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to create a service account User.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountCreateData {
    /// Attributes of the created user.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::ServiceAccountCreateAttributes>,
    /// Relationships of the user object.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::UserRelationships>>,
    /// Users resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UsersType,
}

impl ServiceAccountCreateData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::ServiceAccountCreateAttributes>,
        type_: crate::datadogV2::model::UsersType,
    ) -> ServiceAccountCreateData {
        ServiceAccountCreateData {
            attributes,
            relationships: None,
            type_,
        }
    }
}
