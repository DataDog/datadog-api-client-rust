// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentSeverity {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "SEV-1")]
    SEV_1,
    #[serde(rename = "SEV-2")]
    SEV_2,
    #[serde(rename = "SEV-3")]
    SEV_3,
    #[serde(rename = "SEV-4")]
    SEV_4,
    #[serde(rename = "SEV-5")]
    SEV_5,
}

impl ToString for IncidentSeverity {
    fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::SEV_1 => String::from("SEV-1"),
            Self::SEV_2 => String::from("SEV-2"),
            Self::SEV_3 => String::from("SEV-3"),
            Self::SEV_4 => String::from("SEV-4"),
            Self::SEV_5 => String::from("SEV-5"),
        }
    }
}
