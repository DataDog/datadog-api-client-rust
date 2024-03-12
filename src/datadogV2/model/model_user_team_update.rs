// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A user's relationship with a team
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamUpdate {
    /// Team membership attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::UserTeamAttributes>,
    /// Team membership type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserTeamType,
}

impl UserTeamUpdate {
    pub fn new(type_: crate::datadogV2::model::UserTeamType) -> UserTeamUpdate {
        UserTeamUpdate {
            attributes: None,
            type_,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::UserTeamAttributes) -> Self {
        self.attributes = Some(value);
        self
    }
}
