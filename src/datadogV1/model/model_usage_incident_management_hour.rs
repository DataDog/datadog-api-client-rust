// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident management usage for a given organization for a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageIncidentManagementHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// Contains the total number monthly active users from the start of the given hour's month until the given hour.
    #[serde(
        rename = "monthly_active_users",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub monthly_active_users: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageIncidentManagementHour {
    pub fn new() -> UsageIncidentManagementHour {
        UsageIncidentManagementHour {
            hour: None,
            monthly_active_users: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn monthly_active_users(&mut self, value: Option<i64>) -> &mut Self {
        self.monthly_active_users = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageIncidentManagementHour {
    fn default() -> Self {
        Self::new()
    }
}
