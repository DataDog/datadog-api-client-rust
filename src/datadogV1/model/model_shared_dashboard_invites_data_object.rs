// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the information for an invitation to a shared dashboard.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardInvitesDataObject {
    /// Attributes of the shared dashboard invitation
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV1::model::SharedDashboardInvitesDataObjectAttributes>,
    /// Type for shared dashboard invitation request body.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::DashboardInviteType,
}

impl SharedDashboardInvitesDataObject {
    pub fn new(
        attributes: Box<crate::datadogV1::model::SharedDashboardInvitesDataObjectAttributes>,
        type_: crate::datadogV1::model::DashboardInviteType,
    ) -> SharedDashboardInvitesDataObject {
        SharedDashboardInvitesDataObject { attributes, type_ }
    }
}
