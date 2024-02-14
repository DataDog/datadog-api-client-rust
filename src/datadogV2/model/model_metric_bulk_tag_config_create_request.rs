// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper object for a single bulk tag configuration request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigCreateRequest {
    /// Request object to bulk configure tags for metrics matching the given prefix.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::MetricBulkTagConfigCreate,
}

impl MetricBulkTagConfigCreateRequest {
    pub fn new(
        data: crate::datadogV2::model::MetricBulkTagConfigCreate,
    ) -> MetricBulkTagConfigCreateRequest {
        MetricBulkTagConfigCreateRequest { data }
    }
}
