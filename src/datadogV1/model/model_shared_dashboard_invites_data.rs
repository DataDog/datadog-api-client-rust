// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An object or list of objects containing the information for an invitation to a shared dashboard.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SharedDashboardInvitesData {
    SharedDashboardInvitesDataObject(
        Box<crate::datadogV1::model::SharedDashboardInvitesDataObject>,
    ),
    SharedDashboardInvitesDataList(Vec<crate::datadogV1::model::SharedDashboardInvitesDataObject>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SharedDashboardInvitesData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SharedDashboardInvitesDataObject>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SharedDashboardInvitesData::SharedDashboardInvitesDataObject(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV1::model::SharedDashboardInvitesDataObject>,
        >(value.clone())
        {
            return Ok(SharedDashboardInvitesData::SharedDashboardInvitesDataList(
                _v,
            ));
        }

        return Ok(SharedDashboardInvitesData::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
