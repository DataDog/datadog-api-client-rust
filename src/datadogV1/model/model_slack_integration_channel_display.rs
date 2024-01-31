// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration options for what is shown in an alert event message.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationChannelDisplay {
    /// Show the main body of the alert event.
    #[serde(rename = "message")]
    pub message: Option<bool>,
    /// Show the list of @-handles in the alert event.
    #[serde(rename = "notified")]
    pub notified: Option<bool>,
    /// Show the alert event's snapshot image.
    #[serde(rename = "snapshot")]
    pub snapshot: Option<bool>,
    /// Show the scopes on which the monitor alerted.
    #[serde(rename = "tags")]
    pub tags: Option<bool>,
}

impl SlackIntegrationChannelDisplay {
    pub fn new() -> SlackIntegrationChannelDisplay {
        SlackIntegrationChannelDisplay {
            message: None,
            notified: None,
            snapshot: None,
            tags: None,
        }
    }

    pub fn with_message(&mut self, value: bool) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn with_notified(&mut self, value: bool) -> &mut Self {
        self.notified = Some(value);
        self
    }

    pub fn with_snapshot(&mut self, value: bool) -> &mut Self {
        self.snapshot = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: bool) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
impl Default for SlackIntegrationChannelDisplay {
    fn default() -> Self {
        Self::new()
    }
}
