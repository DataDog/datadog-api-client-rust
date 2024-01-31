// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A created EventBridge source.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeCreateResponse {
    /// The event source name.
    #[serde(rename = "event_source_name")]
    pub event_source_name: Option<String>,
    /// True if the event bus was created in addition to the source.
    #[serde(rename = "has_bus")]
    pub has_bus: Option<bool>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The event source status "created".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::AWSEventBridgeCreateStatus>,
}

impl AWSEventBridgeCreateResponse {
    pub fn new() -> AWSEventBridgeCreateResponse {
        AWSEventBridgeCreateResponse {
            event_source_name: None,
            has_bus: None,
            region: None,
            status: None,
        }
    }

    pub fn with_event_source_name(&mut self, value: String) -> &mut Self {
        self.event_source_name = Some(value);
        self
    }

    pub fn with_has_bus(&mut self, value: bool) -> &mut Self {
        self.has_bus = Some(value);
        self
    }

    pub fn with_region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    pub fn with_status(
        &mut self,
        value: crate::datadogV1::model::AWSEventBridgeCreateStatus,
    ) -> &mut Self {
        self.status = Some(value);
        self
    }
}
impl Default for AWSEventBridgeCreateResponse {
    fn default() -> Self {
        Self::new()
    }
}
