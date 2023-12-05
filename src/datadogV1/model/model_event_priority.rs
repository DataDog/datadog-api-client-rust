// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventPriority {
    #[serde(rename = "normal")]
    NORMAL,
    #[serde(rename = "low")]
    LOW,
}

impl ToString for EventPriority {
    fn to_string(&self) -> String {
        match self {
            Self::NORMAL => String::from("normal"),
            Self::LOW => String::from("low"),
        }
    }
}

impl Default for EventPriority {
    fn default() -> EventPriority {
        Self::NORMAL
    }
}