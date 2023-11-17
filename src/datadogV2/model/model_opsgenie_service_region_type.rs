// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OpsgenieServiceRegionType {
    #[serde(rename = "us")]
    US,
    #[serde(rename = "eu")]
    EU,
    #[serde(rename = "custom")]
    CUSTOM,
}

impl ToString for OpsgenieServiceRegionType {
    fn to_string(&self) -> String {
        match self {
            Self::US => String::from("us"),
            Self::EU => String::from("eu"),
            Self::CUSTOM => String::from("custom"),
        }
    }
}

impl Default for OpsgenieServiceRegionType {
    fn default() -> OpsgenieServiceRegionType {
        Self::US
    }
}
