// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleQueryAggregation {
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "cardinality")]
    CARDINALITY,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "new_value")]
    NEW_VALUE,
    #[serde(rename = "geo_data")]
    GEO_DATA,
    #[serde(rename = "event_count")]
    EVENT_COUNT,
    #[serde(rename = "none")]
    NONE,
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
        }
    }
}
