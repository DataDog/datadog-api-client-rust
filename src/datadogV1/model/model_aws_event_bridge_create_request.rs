// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object used to create an EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeCreateRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// True if Datadog should create the event bus in addition to the event
    /// source. Requires the `events:CreateEventBus` permission.
    #[serde(rename = "create_event_bus")]
    pub create_event_bus: Option<bool>,
    /// The given part of the event source name, which is then combined with an
    /// assigned suffix to form the full name.
    #[serde(rename = "event_generator_name")]
    pub event_generator_name: Option<String>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
}

impl AWSEventBridgeCreateRequest {
    pub fn new() -> AWSEventBridgeCreateRequest {
        AWSEventBridgeCreateRequest {
            account_id: None,
            create_event_bus: None,
            event_generator_name: None,
            region: None,
        }
    }

    pub fn account_id(&mut self, value: String) -> &mut Self {
        self.account_id = Some(value);
        self
    }

    pub fn create_event_bus(&mut self, value: bool) -> &mut Self {
        self.create_event_bus = Some(value);
        self
    }

    pub fn event_generator_name(&mut self, value: String) -> &mut Self {
        self.event_generator_name = Some(value);
        self
    }

    pub fn region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }
}

impl Default for AWSEventBridgeCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}
