// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogsComputeType {
    #[serde(rename = "timeseries")]
    TIMESERIES,
    #[serde(rename = "total")]
    TOTAL,
}

impl ToString for LogsComputeType {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESERIES => String::from("timeseries"),
            Self::TOTAL => String::from("total"),
        }
    }
}
