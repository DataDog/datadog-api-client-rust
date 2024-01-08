// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attribute object associated with the SLO correction to be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionUpdateRequestAttributes {
    /// Category the SLO correction belongs to.
    #[serde(rename = "category")]
    pub category: Option<crate::datadogV1::model::SLOCorrectionCategory>,
    /// Description of the correction being made.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Length of time (in seconds) for a specified `rrule` recurring SLO correction.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Ending time of the correction in epoch seconds.
    #[serde(rename = "end")]
    pub end: Option<i64>,
    /// The recurrence rules as defined in the iCalendar RFC 5545. The supported rules for SLO corrections
    /// are `FREQ`, `INTERVAL`, `COUNT`, and `UNTIL`.
    #[serde(rename = "rrule")]
    pub rrule: Option<String>,
    /// Starting time of the correction in epoch seconds.
    #[serde(rename = "start")]
    pub start: Option<i64>,
    /// The timezone to display in the UI for the correction times (defaults to "UTC").
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

impl SLOCorrectionUpdateRequestAttributes {
    pub fn new() -> SLOCorrectionUpdateRequestAttributes {
        SLOCorrectionUpdateRequestAttributes {
            category: None,
            description: None,
            duration: None,
            end: None,
            rrule: None,
            start: None,
            timezone: None,
        }
    }
}
impl Default for SLOCorrectionUpdateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}
