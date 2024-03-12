// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A notification triggered by the monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResultNotification {
    /// The email address that received the notification.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// The username receiving the notification
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl MonitorSearchResultNotification {
    pub fn new() -> MonitorSearchResultNotification {
        MonitorSearchResultNotification {
            handle: None,
            name: None,
        }
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for MonitorSearchResultNotification {
    fn default() -> Self {
        Self::new()
    }
}
