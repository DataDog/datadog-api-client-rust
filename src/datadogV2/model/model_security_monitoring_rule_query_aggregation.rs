// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleQueryAggregation {
    COUNT,
    CARDINALITY,
    SUM,
    MAX,
    NEW_VALUE,
    GEO_DATA,
    EVENT_COUNT,
    NONE,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SecurityMonitoringRuleQueryAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT => String::from("count"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::SUM => String::from("sum"),
            Self::MAX => String::from("max"),
            Self::NEW_VALUE => String::from("new_value"),
            Self::GEO_DATA => String::from("geo_data"),
            Self::EVENT_COUNT => String::from("event_count"),
            Self::NONE => String::from("none"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleQueryAggregation {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleQueryAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "count" => Self::COUNT,
            "cardinality" => Self::CARDINALITY,
            "sum" => Self::SUM,
            "max" => Self::MAX,
            "new_value" => Self::NEW_VALUE,
            "geo_data" => Self::GEO_DATA,
            "event_count" => Self::EVENT_COUNT,
            "none" => Self::NONE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
