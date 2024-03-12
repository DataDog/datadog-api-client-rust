// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object with all test events matching the request and pagination information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppTestEventsResponse {
    /// Array of events matching the request.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::CIAppTestEvent>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::CIAppResponseLinks>,
    /// The metadata associated with a request.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CIAppResponseMetadataWithPagination>,
}

impl CIAppTestEventsResponse {
    pub fn new() -> CIAppTestEventsResponse {
        CIAppTestEventsResponse {
            data: None,
            links: None,
            meta: None,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV2::model::CIAppTestEvent>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn links(mut self, value: crate::datadogV2::model::CIAppResponseLinks) -> Self {
        self.links = Some(value);
        self
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::CIAppResponseMetadataWithPagination,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for CIAppTestEventsResponse {
    fn default() -> Self {
        Self::new()
    }
}
