// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A user or role assignment related to an incident responder.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IncidentResponderIncludedItem {
    IncidentUserData(Box<crate::datadogV2::model::IncidentUserData>),
    IncidentRoleAssignmentDataResponse(
        Box<crate::datadogV2::model::IncidentRoleAssignmentDataResponse>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for IncidentResponderIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::IncidentUserData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentResponderIncludedItem::IncidentUserData(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IncidentRoleAssignmentDataResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentResponderIncludedItem::IncidentRoleAssignmentDataResponse(_v));
            }
        }

        return Ok(IncidentResponderIncludedItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
