// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaAndFunctionEventAggregation {
    COUNT,
    CARDINALITY,
    MEDIAN,
    PC75,
    PC90,
    PC95,
    PC98,
    PC99,
    SUM,
    MIN,
    MAX,
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

impl Serialize for FormulaAndFunctionEventAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionEventAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "count" => Self::COUNT,
            "cardinality" => Self::CARDINALITY,
            "median" => Self::MEDIAN,
            "pc75" => Self::PC75,
            "pc90" => Self::PC90,
            "pc95" => Self::PC95,
            "pc98" => Self::PC98,
            "pc99" => Self::PC99,
            "sum" => Self::SUM,
            "min" => Self::MIN,
            "max" => Self::MAX,
            "avg" => Self::AVG,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
