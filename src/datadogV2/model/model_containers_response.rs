// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of containers.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainersResponse {
    /// Array of Container objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ContainerItem>>,
    /// Pagination links.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::ContainersResponseLinks>,
    /// Response metadata object.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ContainerMeta>,
}

impl ContainersResponse {
    pub fn new() -> ContainersResponse {
        ContainersResponse {
            data: None,
            links: None,
            meta: None,
        }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::ContainerItem>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn links(&mut self, value: crate::datadogV2::model::ContainersResponseLinks) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::ContainerMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for ContainersResponse {
    fn default() -> Self {
        Self::new()
    }
}
