// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetricsAggregator {
    AVG,
    MIN,
    MAX,
    SUM,
    LAST,
    PERCENTILE,
    MEAN,
    L2NORM,
    AREA,
    UnparsedObject(crate::datadog::UnparsedObejct),
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
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MetricsAggregator {
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

impl<'de> Deserialize<'de> for MetricsAggregator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => Self::AVG,
            "min" => Self::MIN,
            "max" => Self::MAX,
            "sum" => Self::SUM,
            "last" => Self::LAST,
            "percentile" => Self::PERCENTILE,
            "mean" => Self::MEAN,
            "l2norm" => Self::L2NORM,
            "area" => Self::AREA,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
