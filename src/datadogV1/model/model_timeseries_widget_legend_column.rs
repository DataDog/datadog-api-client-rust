// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TimeseriesWidgetLegendColumn {
    #[serde(rename = "value")]
    VALUE,
    #[serde(rename = "avg")]
    AVG,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "max")]
    MAX,
}

impl ToString for TimeseriesWidgetLegendColumn {
    fn to_string(&self) -> String {
        match self {
            Self::VALUE => String::from("value"),
            Self::AVG => String::from("avg"),
            Self::SUM => String::from("sum"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
        }
    }
}
