// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MonitorFormulaAndFunctionCostAggregator {
    AVG,
    SUM,
    MAX,
    MIN,
    LAST,
    AREA,
    L2NORM,
    PERCENTILE,
    STDDEV,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for MonitorFormulaAndFunctionCostAggregator {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::SUM => String::from("sum"),
            Self::MAX => String::from("max"),
            Self::MIN => String::from("min"),
            Self::LAST => String::from("last"),
            Self::AREA => String::from("area"),
            Self::L2NORM => String::from("l2norm"),
            Self::PERCENTILE => String::from("percentile"),
            Self::STDDEV => String::from("stddev"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MonitorFormulaAndFunctionCostAggregator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionCostAggregator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => Self::AVG,
            "sum" => Self::SUM,
            "max" => Self::MAX,
            "min" => Self::MIN,
            "last" => Self::LAST,
            "area" => Self::AREA,
            "l2norm" => Self::L2NORM,
            "percentile" => Self::PERCENTILE,
            "stddev" => Self::STDDEV,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
