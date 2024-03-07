// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetComparator {
    #[serde(rename = "=")]
    EQUAL_TO,
    #[serde(rename = ">")]
    GREATER_THAN,
    #[serde(rename = ">=")]
    GREATER_THAN_OR_EQUAL_TO,
    #[serde(rename = "<")]
    LESS_THAN,
    #[serde(rename = "<=")]
    LESS_THAN_OR_EQUAL_TO,
}

impl ToString for WidgetComparator {
    fn to_string(&self) -> String {
        match self {
            Self::EQUAL_TO => String::from("="),
            Self::GREATER_THAN => String::from(">"),
            Self::GREATER_THAN_OR_EQUAL_TO => String::from(">="),
            Self::LESS_THAN => String::from("<"),
            Self::LESS_THAN_OR_EQUAL_TO => String::from("<="),
        }
    }
}
