// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident integration metadata for the Slack integration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationMetadata {
    /// Array of Slack channels in this integration metadata.
    #[serde(rename = "channels")]
    pub channels: Vec<crate::datadogV2::model::SlackIntegrationMetadataChannelItem>,
}

impl SlackIntegrationMetadata {
    pub fn new(
        channels: Vec<crate::datadogV2::model::SlackIntegrationMetadataChannelItem>,
    ) -> SlackIntegrationMetadata {
        SlackIntegrationMetadata { channels }
    }
}