// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of Container Images.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImagesResponse {
    /// Array of Container Image objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ContainerImageItem>>,
    /// Pagination links.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::ContainerImagesResponseLinks>>,
    /// Response metadata object.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::ContainerImageMeta>>,
}

impl ContainerImagesResponse {
    pub fn new() -> ContainerImagesResponse {
        ContainerImagesResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}
impl Default for ContainerImagesResponse {
    fn default() -> Self {
        Self::new()
    }
}
