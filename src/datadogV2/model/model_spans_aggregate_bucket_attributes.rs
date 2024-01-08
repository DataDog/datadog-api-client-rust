// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A bucket values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAggregateBucketAttributes {
    /// The key, value pairs for each group by.
    #[serde(rename = "by")]
    pub by: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The compute data.
    #[serde(rename = "compute")]
    pub compute: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// A map of the metric name -> value for regular compute or list of values for a timeseries.
    #[serde(rename = "computes")]
    pub computes: Option<
        std::collections::HashMap<String, crate::datadogV2::model::SpansAggregateBucketValue>,
    >,
}

impl SpansAggregateBucketAttributes {
    pub fn new() -> SpansAggregateBucketAttributes {
        SpansAggregateBucketAttributes {
            by: None,
            compute: None,
            computes: None,
        }
    }
}
impl Default for SpansAggregateBucketAttributes {
    fn default() -> Self {
        Self::new()
    }
}
