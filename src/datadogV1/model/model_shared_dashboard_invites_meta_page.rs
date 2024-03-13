// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the total count of invitations across all pages
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardInvitesMetaPage {
    /// The total number of invitations on this shared board, across all pages.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
}

impl SharedDashboardInvitesMetaPage {
    pub fn new() -> SharedDashboardInvitesMetaPage {
        SharedDashboardInvitesMetaPage { total_count: None }
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }
}

impl Default for SharedDashboardInvitesMetaPage {
    fn default() -> Self {
        Self::new()
    }
}
