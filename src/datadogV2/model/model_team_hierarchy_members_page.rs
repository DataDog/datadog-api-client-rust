// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Pagination metadata for the selected pagination mode.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TeamHierarchyMembersPage {
    TeamHierarchyMembersOffsetPage(Box<crate::datadogV2::model::TeamHierarchyMembersOffsetPage>),
    TeamHierarchyMembersNumberPage(Box<crate::datadogV2::model::TeamHierarchyMembersNumberPage>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TeamHierarchyMembersPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TeamHierarchyMembersOffsetPage>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TeamHierarchyMembersPage::TeamHierarchyMembersOffsetPage(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TeamHierarchyMembersNumberPage>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TeamHierarchyMembersPage::TeamHierarchyMembersNumberPage(_v));
            }
        }

        return Ok(TeamHierarchyMembersPage::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
