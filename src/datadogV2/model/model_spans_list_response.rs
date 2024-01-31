// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object with all spans matching the request and pagination information.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansListResponse {
    /// Array of spans matching the request.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::Span>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::SpansListResponseLinks>,
    /// The metadata associated with a request.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SpansListResponseMetadata>,
}

impl SpansListResponse {
    pub fn new() -> SpansListResponse {
        SpansListResponse {
            data: None,
            links: None,
            meta: None,
        }
    }

    pub fn with_data(&mut self, value: Vec<crate::datadogV2::model::Span>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_links(
        &mut self,
        value: crate::datadogV2::model::SpansListResponseLinks,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn with_meta(
        &mut self,
        value: crate::datadogV2::model::SpansListResponseMetadata,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}
impl Default for SpansListResponse {
    fn default() -> Self {
        Self::new()
    }
}
