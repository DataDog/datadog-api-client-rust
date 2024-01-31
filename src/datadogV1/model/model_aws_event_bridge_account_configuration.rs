// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The EventBridge configuration for one AWS account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeAccountConfiguration {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// Array of AWS event sources associated with this account.
    #[serde(rename = "eventHubs")]
    pub event_hubs: Option<Vec<crate::datadogV1::model::AWSEventBridgeSource>>,
    /// Array of tags (in the form `key:value`) which are added to all hosts
    /// and metrics reporting through the main AWS integration.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl AWSEventBridgeAccountConfiguration {
    pub fn new() -> AWSEventBridgeAccountConfiguration {
        AWSEventBridgeAccountConfiguration {
            account_id: None,
            event_hubs: None,
            tags: None,
        }
    }

    pub fn with_account_id(&mut self, value: String) -> &mut Self {
        self.account_id = Some(value);
        self
    }

    pub fn with_event_hubs(
        &mut self,
        value: Vec<crate::datadogV1::model::AWSEventBridgeSource>,
    ) -> &mut Self {
        self.event_hubs = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
impl Default for AWSEventBridgeAccountConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
