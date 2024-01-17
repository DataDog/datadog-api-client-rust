// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attribute object associated with the SLO correction.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionResponseAttributes {
    /// Category the SLO correction belongs to.
    #[serde(rename = "category")]
    pub category: Option<crate::datadogV1::model::SLOCorrectionCategory>,
    /// The epoch timestamp of when the correction was created at.
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub created_at: Option<Option<i64>>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<Box<crate::datadogV1::model::Creator>>,
    /// Description of the correction being made.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Length of time (in seconds) for a specified `rrule` recurring SLO correction.
    #[serde(
        rename = "duration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub duration: Option<Option<i64>>,
    /// Ending time of the correction in epoch seconds.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<i64>>,
    /// The epoch timestamp of when the correction was modified at.
    #[serde(
        rename = "modified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_at: Option<Option<i64>>,
    /// Modifier of the object.
    #[serde(
        rename = "modifier",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modifier:
        Option<Option<Box<crate::datadogV1::model::SLOCorrectionResponseAttributesModifier>>>,
    /// The recurrence rules as defined in the iCalendar RFC 5545. The supported rules for SLO corrections
    /// are `FREQ`, `INTERVAL`, `COUNT`, and `UNTIL`.
    #[serde(rename = "rrule", default, with = "::serde_with::rust::double_option")]
    pub rrule: Option<Option<String>>,
    /// ID of the SLO that this correction applies to.
    #[serde(rename = "slo_id")]
    pub slo_id: Option<String>,
    /// Starting time of the correction in epoch seconds.
    #[serde(rename = "start")]
    pub start: Option<i64>,
    /// The timezone to display in the UI for the correction times (defaults to "UTC").
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

impl SLOCorrectionResponseAttributes {
    pub fn new() -> SLOCorrectionResponseAttributes {
        SLOCorrectionResponseAttributes {
            category: None,
            created_at: None,
            creator: None,
            description: None,
            duration: None,
            end: None,
            modified_at: None,
            modifier: None,
            rrule: None,
            slo_id: None,
            start: None,
            timezone: None,
        }
    }
}
impl Default for SLOCorrectionResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
