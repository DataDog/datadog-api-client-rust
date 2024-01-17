// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricsAggregator {
    #[serde(rename = "avg")]
    AVG,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "last")]
    LAST,
    #[serde(rename = "percentile")]
    PERCENTILE,
    #[serde(rename = "mean")]
    MEAN,
    #[serde(rename = "l2norm")]
    L2NORM,
    #[serde(rename = "area")]
    AREA,
}

impl ToString for MetricsAggregator {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::SUM => String::from("sum"),
            Self::LAST => String::from("last"),
            Self::PERCENTILE => String::from("percentile"),
            Self::MEAN => String::from("mean"),
            Self::L2NORM => String::from("l2norm"),
            Self::AREA => String::from("area"),
        }
    }
}
