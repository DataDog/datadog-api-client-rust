// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApmStatsQueryRowType {
    #[serde(rename = "service")]
    SERVICE,
    #[serde(rename = "resource")]
    RESOURCE,
    #[serde(rename = "span")]
    SPAN,
}

impl ToString for ApmStatsQueryRowType {
    fn to_string(&self) -> String {
        match self {
            Self::SERVICE => String::from("service"),
            Self::RESOURCE => String::from("resource"),
            Self::SPAN => String::from("span"),
        }
    }
}
