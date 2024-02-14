// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableWidgetCellDisplayMode {
    #[serde(rename = "number")]
    NUMBER,
    #[serde(rename = "bar")]
    BAR,
}

impl ToString for TableWidgetCellDisplayMode {
    fn to_string(&self) -> String {
        match self {
            Self::NUMBER => String::from("number"),
            Self::BAR => String::from("bar"),
        }
    }
}
