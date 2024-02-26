// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CIAppAggregationFunction {
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "cardinality")]
    CARDINALITY,
    #[serde(rename = "pc75")]
    PERCENTILE_75,
    #[serde(rename = "pc90")]
    PERCENTILE_90,
    #[serde(rename = "pc95")]
    PERCENTILE_95,
    #[serde(rename = "pc98")]
    PERCENTILE_98,
    #[serde(rename = "pc99")]
    PERCENTILE_99,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "avg")]
    AVG,
    #[serde(rename = "median")]
    MEDIAN,
    #[serde(rename = "latest")]
    LATEST,
    #[serde(rename = "earliest")]
    EARLIEST,
    #[serde(rename = "most_frequent")]
    MOST_FREQUENT,
    #[serde(rename = "delta")]
    DELTA,
}

impl ToString for CIAppAggregationFunction {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT => String::from("count"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::PERCENTILE_75 => String::from("pc75"),
            Self::PERCENTILE_90 => String::from("pc90"),
            Self::PERCENTILE_95 => String::from("pc95"),
            Self::PERCENTILE_98 => String::from("pc98"),
            Self::PERCENTILE_99 => String::from("pc99"),
            Self::SUM => String::from("sum"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::AVG => String::from("avg"),
            Self::MEDIAN => String::from("median"),
            Self::LATEST => String::from("latest"),
            Self::EARLIEST => String::from("earliest"),
            Self::MOST_FREQUENT => String::from("most_frequent"),
            Self::DELTA => String::from("delta"),
        }
    }
}
