// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Invitations data and metadata that exists for a shared dashboard returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardInvites {
    /// An object or list of objects containing the information for an invitation to a shared dashboard.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV1::model::SharedDashboardInvitesData>,
    /// Pagination metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV1::model::SharedDashboardInvitesMeta>>,
}

impl SharedDashboardInvites {
    pub fn new(
        data: Box<crate::datadogV1::model::SharedDashboardInvitesData>,
    ) -> SharedDashboardInvites {
        SharedDashboardInvites { data, meta: None }
    }
}
