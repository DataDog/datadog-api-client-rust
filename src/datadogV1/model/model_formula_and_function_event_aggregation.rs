// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormulaAndFunctionEventAggregation {
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "cardinality")]
    CARDINALITY,
    #[serde(rename = "median")]
    MEDIAN,
    #[serde(rename = "pc75")]
    PC75,
    #[serde(rename = "pc90")]
    PC90,
    #[serde(rename = "pc95")]
    PC95,
    #[serde(rename = "pc98")]
    PC98,
    #[serde(rename = "pc99")]
    PC99,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "avg")]
    AVG,
}

impl ToString for FormulaAndFunctionEventAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT => String::from("count"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::MEDIAN => String::from("median"),
            Self::PC75 => String::from("pc75"),
            Self::PC90 => String::from("pc90"),
            Self::PC95 => String::from("pc95"),
            Self::PC98 => String::from("pc98"),
            Self::PC99 => String::from("pc99"),
            Self::SUM => String::from("sum"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::AVG => String::from("avg"),
        }
    }
}
