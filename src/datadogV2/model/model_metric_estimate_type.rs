// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MetricEstimateType {
    #[serde(rename = "count_or_gauge")]
    COUNT_OR_GAUGE,
    #[serde(rename = "distribution")]
    DISTRIBUTION,
    #[serde(rename = "percentile")]
    PERCENTILE,
}

impl ToString for MetricEstimateType {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT_OR_GAUGE => String::from("count_or_gauge"),
            Self::DISTRIBUTION => String::from("distribution"),
            Self::PERCENTILE => String::from("percentile"),
        }
    }
}
