// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UsageSort {
    #[serde(rename = "computed_on")]
    COMPUTED_ON,
    #[serde(rename = "size")]
    SIZE,
    #[serde(rename = "start_date")]
    START_DATE,
    #[serde(rename = "end_date")]
    END_DATE,
}

impl ToString for UsageSort {
    fn to_string(&self) -> String {
        match self {
            Self::COMPUTED_ON => String::from("computed_on"),
            Self::SIZE => String::from("size"),
            Self::START_DATE => String::from("start_date"),
            Self::END_DATE => String::from("end_date"),
        }
    }
}
