// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing timeframes and timezone used for advanced scheduling.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsScheduling {
    /// Array containing objects describing the scheduling pattern to apply to each day.
    #[serde(rename = "timeframes")]
    pub timeframes: Option<Vec<crate::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe>>,
    /// Timezone in which the timeframe is based.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

impl SyntheticsTestOptionsScheduling {
    pub fn new() -> SyntheticsTestOptionsScheduling {
        SyntheticsTestOptionsScheduling {
            timeframes: None,
            timezone: None,
        }
    }

    pub fn with_timeframes(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe>,
    ) -> &mut Self {
        self.timeframes = Some(value);
        self
    }

    pub fn with_timezone(&mut self, value: String) -> &mut Self {
        self.timezone = Some(value);
        self
    }
}
impl Default for SyntheticsTestOptionsScheduling {
    fn default() -> Self {
        Self::new()
    }
}
