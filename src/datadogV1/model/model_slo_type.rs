// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SLOType {
    #[serde(rename = "metric")]
    METRIC,
    #[serde(rename = "monitor")]
    MONITOR,
}

impl ToString for SLOType {
    fn to_string(&self) -> String {
        match self {
            Self::METRIC => String::from("metric"),
            Self::MONITOR => String::from("monitor"),
        }
    }
}
