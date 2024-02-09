// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Slack channel configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationChannel {
    /// Configuration options for what is shown in an alert event message.
    #[serde(rename = "display")]
    pub display: Option<crate::datadogV1::model::SlackIntegrationChannelDisplay>,
    /// Your channel name.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl SlackIntegrationChannel {
    pub fn new() -> SlackIntegrationChannel {
        SlackIntegrationChannel {
            display: None,
            name: None,
        }
    }

    pub fn display(
        &mut self,
        value: crate::datadogV1::model::SlackIntegrationChannelDisplay,
    ) -> &mut Self {
        self.display = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for SlackIntegrationChannel {
    fn default() -> Self {
        Self::new()
    }
}
