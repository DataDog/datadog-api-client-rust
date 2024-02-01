// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Bucket values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppTestsBucketResponse {
    /// The key-value pairs for each group-by.
    #[serde(rename = "by")]
    pub by: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A map of the metric name to value for regular compute, or a list of values for a timeseries.
    #[serde(rename = "computes")]
    pub computes: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::CIAppAggregateBucketValue>,
    >,
}

impl CIAppTestsBucketResponse {
    pub fn new() -> CIAppTestsBucketResponse {
        CIAppTestsBucketResponse {
            by: None,
            computes: None,
        }
    }

    pub fn by(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.by = Some(value);
        self
    }

    pub fn computes(
        &mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::CIAppAggregateBucketValue,
        >,
    ) -> &mut Self {
        self.computes = Some(value);
        self
    }
}

impl Default for CIAppTestsBucketResponse {
    fn default() -> Self {
        Self::new()
    }
}
