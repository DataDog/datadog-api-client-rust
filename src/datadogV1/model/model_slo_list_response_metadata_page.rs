// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing information about the pages of the list of SLOs.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListResponseMetadataPage {
    /// The total number of resources that could be retrieved ignoring the parameters and filters in the request.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    /// The total number of resources that match the parameters and filters in the request. This attribute can be used by a client to determine the total number of pages.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
}

impl SLOListResponseMetadataPage {
    pub fn new() -> SLOListResponseMetadataPage {
        SLOListResponseMetadataPage {
            total_count: None,
            total_filtered_count: None,
        }
    }

    pub fn with_total_count(&mut self, value: i64) -> &mut Self {
        self.total_count = Some(value);
        self
    }

    pub fn with_total_filtered_count(&mut self, value: i64) -> &mut Self {
        self.total_filtered_count = Some(value);
        self
    }
}
impl Default for SLOListResponseMetadataPage {
    fn default() -> Self {
        Self::new()
    }
}
