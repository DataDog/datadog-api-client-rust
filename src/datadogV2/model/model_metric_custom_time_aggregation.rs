// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MetricCustomTimeAggregation {
    #[serde(rename = "avg")]
    AVG,
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "sum")]
    SUM,
}

impl ToString for MetricCustomTimeAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::COUNT => String::from("count"),
            Self::MAX => String::from("max"),
            Self::MIN => String::from("min"),
            Self::SUM => String::from("sum"),
        }
    }
}
