// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Bucket values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMBucketResponse {
    /// The key-value pairs for each group-by.
    #[serde(rename = "by")]
    pub by: Option<std::collections::HashMap<String, String>>,
    /// A map of the metric name to value for regular compute, or a list of values for a timeseries.
    #[serde(rename = "computes")]
    pub computes:
        Option<std::collections::HashMap<String, crate::datadogV2::model::RUMAggregateBucketValue>>,
}

impl RUMBucketResponse {
    pub fn new() -> RUMBucketResponse {
        RUMBucketResponse {
            by: None,
            computes: None,
        }
    }
}