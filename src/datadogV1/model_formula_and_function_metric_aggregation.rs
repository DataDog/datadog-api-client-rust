// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormulaAndFunctionMetricAggregation {
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
    #[serde(rename = "area")]
	AREA,
    #[serde(rename = "l2norm")]
	L2NORM,
    #[serde(rename = "percentile")]
	PERCENTILE,
}

impl ToString for FormulaAndFunctionMetricAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::SUM => String::from("sum"),
            Self::LAST => String::from("last"),
            Self::AREA => String::from("area"),
            Self::L2NORM => String::from("l2norm"),
            Self::PERCENTILE => String::from("percentile"),
        }
    }
}

impl Default for FormulaAndFunctionMetricAggregation {
    fn default() -> FormulaAndFunctionMetricAggregation {
        Self::AVG
    }
}
