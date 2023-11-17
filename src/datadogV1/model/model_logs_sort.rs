// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsSort {
    #[serde(rename = "asc")]
    TIME_ASCENDING,
    #[serde(rename = "desc")]
    TIME_DESCENDING,
}

impl ToString for LogsSort {
    fn to_string(&self) -> String {
        match self {
            Self::TIME_ASCENDING => String::from("asc"),
            Self::TIME_DESCENDING => String::from("desc"),
        }
    }
}

impl Default for LogsSort {
    fn default() -> LogsSort {
        Self::TIME_ASCENDING
    }
}
