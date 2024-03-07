// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScatterplotWidgetAggregator {
    #[serde(rename = "avg")]
    AVERAGE,
    #[serde(rename = "last")]
    LAST,
    #[serde(rename = "max")]
    MAXIMUM,
    #[serde(rename = "min")]
    MINIMUM,
    #[serde(rename = "sum")]
    SUM,
}

impl ToString for ScatterplotWidgetAggregator {
    fn to_string(&self) -> String {
        match self {
            Self::AVERAGE => String::from("avg"),
            Self::LAST => String::from("last"),
            Self::MAXIMUM => String::from("max"),
            Self::MINIMUM => String::from("min"),
            Self::SUM => String::from("sum"),
        }
    }
}
