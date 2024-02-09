// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A notification handle that will be notified at incident creation.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentNotificationHandle {
    /// The name of the notified handle.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// The email address used for the notification.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
}

impl IncidentNotificationHandle {
    pub fn new() -> IncidentNotificationHandle {
        IncidentNotificationHandle {
            display_name: None,
            handle: None,
        }
    }

    pub fn display_name(&mut self, value: String) -> &mut Self {
        self.display_name = Some(value);
        self
    }

    pub fn handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }
}

impl Default for IncidentNotificationHandle {
    fn default() -> Self {
        Self::new()
    }
}
