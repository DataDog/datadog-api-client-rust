// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An included user or the requested team, distinguished by resource type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TeamHierarchyMemberIncluded {
    TeamHierarchyMemberUser(Box<crate::datadogV2::model::TeamHierarchyMemberUser>),
    TeamHierarchyMemberTeam(Box<crate::datadogV2::model::TeamHierarchyMemberTeam>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TeamHierarchyMemberIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TeamHierarchyMemberUser>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TeamHierarchyMemberIncluded::TeamHierarchyMemberUser(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TeamHierarchyMemberTeam>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TeamHierarchyMemberIncluded::TeamHierarchyMemberTeam(_v));
            }
        }

        return Ok(TeamHierarchyMemberIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
