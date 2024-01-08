// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object for the spans aggregate API endpoint.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAggregateResponse {
    /// The list of matching buckets, one item per bucket.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::SpansAggregateBucket>>,
    /// The metadata associated with a request.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::SpansAggregateResponseMetadata>>,
}

impl SpansAggregateResponse {
    pub fn new() -> SpansAggregateResponse {
        SpansAggregateResponse {
            data: None,
            meta: None,
        }
    }
}
impl Default for SpansAggregateResponse {
    fn default() -> Self {
        Self::new()
    }
}
