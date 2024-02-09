// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object representing a given user entity.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringTriageUser {
    /// The handle for this user account.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// Numerical ID assigned by Datadog to this user account.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The name for this user account.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// UUID assigned by Datadog to this user account.
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl SecurityMonitoringTriageUser {
    pub fn new(uuid: String) -> SecurityMonitoringTriageUser {
        SecurityMonitoringTriageUser {
            handle: None,
            icon: None,
            id: None,
            name: None,
            uuid,
        }
    }

    pub fn handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }

    pub fn icon(&mut self, value: String) -> &mut Self {
        self.icon = Some(value);
        self
    }

    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn name(&mut self, value: Option<String>) -> &mut Self {
        self.name = Some(value);
        self
    }
}
