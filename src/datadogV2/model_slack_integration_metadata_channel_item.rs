// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationMetadataChannelItem {
    /// Slack channel ID.
    #[serde(rename = "channel_id", skip_serializing_if = "Option::is_none")]
    pub channel_id: String,
    /// Name of the Slack channel.
    #[serde(rename = "channel_name", skip_serializing_if = "Option::is_none")]
    pub channel_name: String,
    /// URL redirecting to the Slack channel.
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: String,
    /// Slack team ID.
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: String,
}

