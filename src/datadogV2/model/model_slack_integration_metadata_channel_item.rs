// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Item in the Slack integration metadata channel array.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationMetadataChannelItem {
    /// Slack channel ID.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// Name of the Slack channel.
    #[serde(rename = "channel_name")]
    pub channel_name: String,
    /// URL redirecting to the Slack channel.
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    /// Slack team ID.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
}

impl SlackIntegrationMetadataChannelItem {
    pub fn new(
        channel_id: String,
        channel_name: String,
        redirect_url: String,
    ) -> SlackIntegrationMetadataChannelItem {
        SlackIntegrationMetadataChannelItem {
            channel_id,
            channel_name,
            redirect_url,
            team_id: None,
        }
    }

    pub fn with_team_id(&mut self, value: String) -> &mut Self {
        self.team_id = Some(value);
        self
    }
}
