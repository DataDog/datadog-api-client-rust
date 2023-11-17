// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpansMetricComputeAggregationType {
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "distribution")]
    DISTRIBUTION,
}

impl ToString for SpansMetricComputeAggregationType {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT => String::from("count"),
            Self::DISTRIBUTION => String::from("distribution"),
        }
    }
}

impl Default for SpansMetricComputeAggregationType {
    fn default() -> SpansMetricComputeAggregationType {
        Self::COUNT
    }
}
