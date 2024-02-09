// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The query results.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMAggregationBucketsResponse {
    /// The list of matching buckets, one item per bucket.
    #[serde(rename = "buckets")]
    pub buckets: Option<Vec<crate::datadogV2::model::RUMBucketResponse>>,
}

impl RUMAggregationBucketsResponse {
    pub fn new() -> RUMAggregationBucketsResponse {
        RUMAggregationBucketsResponse { buckets: None }
    }

    pub fn buckets(&mut self, value: Vec<crate::datadogV2::model::RUMBucketResponse>) -> &mut Self {
        self.buckets = Some(value);
        self
    }
}

impl Default for RUMAggregationBucketsResponse {
    fn default() -> Self {
        Self::new()
    }
}
