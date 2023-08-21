// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardInvites {
    /// An object or list of objects containing the information for an invitation to a shared dashboard.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: SharedDashboardInvitesData,
    /// Pagination metadata returned by the API.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: SharedDashboardInvitesMeta,
}

