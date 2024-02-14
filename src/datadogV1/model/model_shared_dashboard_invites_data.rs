// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// An object or list of objects containing the information for an invitation to a shared dashboard.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SharedDashboardInvitesData {
    SharedDashboardInvitesDataObject(
        Box<crate::datadogV1::model::SharedDashboardInvitesDataObject>,
    ),
    SharedDashboardInvitesDataList(Vec<crate::datadogV1::model::SharedDashboardInvitesDataObject>),
}
