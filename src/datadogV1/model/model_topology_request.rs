// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request that will return nodes and edges to be used by topology map.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologyRequest {
    /// Query to service-based topology data sources like the service map or data streams.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::TopologyQuery>,
    /// Widget request type.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV1::model::TopologyRequestType>,
}

impl TopologyRequest {
    pub fn new() -> TopologyRequest {
        TopologyRequest {
            query: None,
            request_type: None,
        }
    }

    pub fn query(&mut self, value: crate::datadogV1::model::TopologyQuery) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn request_type(
        &mut self,
        value: crate::datadogV1::model::TopologyRequestType,
    ) -> &mut Self {
        self.request_type = Some(value);
        self
    }
}

impl Default for TopologyRequest {
    fn default() -> Self {
        Self::new()
    }
}
