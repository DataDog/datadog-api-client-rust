// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpansMetricType {
    #[serde(rename = "spans_metrics")]
    SPANS_METRICS,
}

impl ToString for SpansMetricType {
    fn to_string(&self) -> String {
        match self {
            Self::SPANS_METRICS => String::from("spans_metrics"),
        }
    }
}

impl Default for SpansMetricType {
    fn default() -> SpansMetricType {
        Self::SPANS_METRICS
    }
}