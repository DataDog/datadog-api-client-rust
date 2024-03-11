// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Included resources related to the team
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TeamIncluded {
    User(Box<crate::datadogV2::model::User>),
    TeamLink(Box<crate::datadogV2::model::TeamLink>),
    UserTeamPermission(Box<crate::datadogV2::model::UserTeamPermission>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for TeamIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::User>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(TeamIncluded::User(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::TeamLink>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(TeamIncluded::TeamLink(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::UserTeamPermission>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(TeamIncluded::UserTeamPermission(_v));
            }
        }

        return Ok(TeamIncluded::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
