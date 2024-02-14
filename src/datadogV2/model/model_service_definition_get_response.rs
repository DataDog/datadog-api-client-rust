// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Get service definition response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionGetResponse {
    /// Service definition data.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::ServiceDefinitionData>,
}

impl ServiceDefinitionGetResponse {
    pub fn new() -> ServiceDefinitionGetResponse {
        ServiceDefinitionGetResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::ServiceDefinitionData) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for ServiceDefinitionGetResponse {
    fn default() -> Self {
        Self::new()
    }
}
