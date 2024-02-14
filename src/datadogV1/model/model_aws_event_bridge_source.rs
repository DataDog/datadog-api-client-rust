// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeSource {
    /// The event source name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
}

impl AWSEventBridgeSource {
    pub fn new() -> AWSEventBridgeSource {
        AWSEventBridgeSource {
            name: None,
            region: None,
        }
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }
}

impl Default for AWSEventBridgeSource {
    fn default() -> Self {
        Self::new()
    }
}
