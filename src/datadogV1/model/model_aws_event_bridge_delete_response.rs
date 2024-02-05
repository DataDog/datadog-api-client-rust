// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An indicator of the successful deletion of an EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSEventBridgeDeleteResponse {
    /// The event source status "empty".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::AWSEventBridgeDeleteStatus>,
}

impl AWSEventBridgeDeleteResponse {
    pub fn new() -> AWSEventBridgeDeleteResponse {
        AWSEventBridgeDeleteResponse { status: None }
    }

    pub fn status(
        &mut self,
        value: crate::datadogV1::model::AWSEventBridgeDeleteStatus,
    ) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for AWSEventBridgeDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}
