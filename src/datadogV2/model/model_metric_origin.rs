// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metric origin information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricOrigin {
    /// The origin metric type code
    #[serde(rename = "metric_type")]
    pub metric_type: Option<i32>,
    /// The origin product code
    #[serde(rename = "product")]
    pub product: Option<i32>,
    /// The origin service code
    #[serde(rename = "service")]
    pub service: Option<i32>,
}

impl MetricOrigin {
    pub fn new() -> MetricOrigin {
        MetricOrigin {
            metric_type: None,
            product: None,
            service: None,
        }
    }

    pub fn metric_type(&mut self, value: i32) -> &mut Self {
        self.metric_type = Some(value);
        self
    }

    pub fn product(&mut self, value: i32) -> &mut Self {
        self.product = Some(value);
        self
    }

    pub fn service(&mut self, value: i32) -> &mut Self {
        self.service = Some(value);
        self
    }
}

impl Default for MetricOrigin {
    fn default() -> Self {
        Self::new()
    }
}
