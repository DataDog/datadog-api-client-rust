// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A bucket values
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateBucket {
    /// The key, value pairs for each group by
    #[serde(rename = "by")]
    pub by: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A map of the metric name -> value for regular compute or list of values for a timeseries
    #[serde(rename = "computes")]
    pub computes: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::LogsAggregateBucketValue>,
    >,
}

impl LogsAggregateBucket {
    pub fn new() -> LogsAggregateBucket {
        LogsAggregateBucket {
            by: None,
            computes: None,
        }
    }

    pub fn with_by(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.by = Some(value);
        self
    }

    pub fn with_computes(
        &mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::LogsAggregateBucketValue,
        >,
    ) -> &mut Self {
        self.computes = Some(value);
        self
    }
}
impl Default for LogsAggregateBucket {
    fn default() -> Self {
        Self::new()
    }
}
