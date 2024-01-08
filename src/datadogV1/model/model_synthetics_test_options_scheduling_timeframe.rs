// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing a timeframe.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsSchedulingTimeframe {
    /// Number representing the day of the week.
    #[serde(rename = "day")]
    pub day: Option<i32>,
    /// The hour of the day on which scheduling starts.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// The hour of the day on which scheduling ends.
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl SyntheticsTestOptionsSchedulingTimeframe {
    pub fn new() -> SyntheticsTestOptionsSchedulingTimeframe {
        SyntheticsTestOptionsSchedulingTimeframe {
            day: None,
            from: None,
            to: None,
        }
    }
}
impl Default for SyntheticsTestOptionsSchedulingTimeframe {
    fn default() -> Self {
        Self::new()
    }
}
