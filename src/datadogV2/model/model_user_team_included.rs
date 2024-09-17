// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Included resources related to the team membership
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UserTeamIncluded {
    User(Box<crate::datadogV2::model::User>),
    Team(Box<crate::datadogV2::model::Team>),
    AbbreviatedTeam(Box<crate::datadogV2::model::AbbreviatedTeam>),
    UserTeamUser(Box<crate::datadogV2::model::UserTeamUser>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for UserTeamIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::User>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(UserTeamIncluded::User(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::Team>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(UserTeamIncluded::Team(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AbbreviatedTeam>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(UserTeamIncluded::AbbreviatedTeam(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::UserTeamUser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(UserTeamIncluded::UserTeamUser(_v));
            }
        }

        return Ok(UserTeamIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
