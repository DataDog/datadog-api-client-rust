// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FormulaAndFunctionResponseFormat {
    #[serde(rename = "timeseries")]
    TIMESERIES,
    #[serde(rename = "scalar")]
    SCALAR,
    #[serde(rename = "event_list")]
    EVENT_LIST,
}

impl ToString for FormulaAndFunctionResponseFormat {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESERIES => String::from("timeseries"),
            Self::SCALAR => String::from("scalar"),
            Self::EVENT_LIST => String::from("event_list"),
        }
    }
}
