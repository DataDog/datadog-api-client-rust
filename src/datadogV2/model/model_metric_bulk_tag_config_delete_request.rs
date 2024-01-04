// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper object for a single bulk tag deletion request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigDeleteRequest {
    /// Request object to bulk delete all tag configurations for metrics matching the given prefix.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::MetricBulkTagConfigDelete>,
}

impl MetricBulkTagConfigDeleteRequest {
    pub fn new(
        data: Box<crate::datadogV2::model::MetricBulkTagConfigDelete>,
    ) -> MetricBulkTagConfigDeleteRequest {
        MetricBulkTagConfigDeleteRequest { data }
    }
}
