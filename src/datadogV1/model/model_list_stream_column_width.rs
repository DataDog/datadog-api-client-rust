// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ListStreamColumnWidth {
    #[serde(rename = "auto")]
    AUTO,
    #[serde(rename = "compact")]
    COMPACT,
    #[serde(rename = "full")]
    FULL,
}

impl ToString for ListStreamColumnWidth {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::COMPACT => String::from("compact"),
            Self::FULL => String::from("full"),
        }
    }
}
