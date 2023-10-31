// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object used to delete an EventBridge source.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeDeleteRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The event source name.
    #[serde(rename = "event_generator_name")]
    pub event_generator_name: Option<String>,
    /// The event source's [AWS region](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints).
    #[serde(rename = "region")]
    pub region: Option<String>,
}

impl AWSEventBridgeDeleteRequest {
    pub fn new() -> AWSEventBridgeDeleteRequest {
        AWSEventBridgeDeleteRequest {
            account_id: None,
            event_generator_name: None,
            region: None,
        }
    }
}
