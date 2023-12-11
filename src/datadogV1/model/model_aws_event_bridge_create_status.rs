// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AWSEventBridgeCreateStatus {
    #[serde(rename = "created")]
    CREATED,
}

impl ToString for AWSEventBridgeCreateStatus {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED => String::from("created"),
        }
    }
}

impl Default for AWSEventBridgeCreateStatus {
    fn default() -> AWSEventBridgeCreateStatus {
        Self::CREATED
    }
}
