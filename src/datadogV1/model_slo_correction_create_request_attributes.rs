// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionCreateRequestAttributes {
    /// Category the SLO correction belongs to.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: SLOCorrectionCategory,
    /// Description of the correction being made.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Length of time (in seconds) for a specified `rrule` recurring SLO correction.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: i64,
    /// Ending time of the correction in epoch seconds.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: i64,
    /// The recurrence rules as defined in the iCalendar RFC 5545. The supported rules for SLO corrections
are `FREQ`, `INTERVAL`, `COUNT` and `UNTIL`.
    #[serde(rename = "rrule", skip_serializing_if = "Option::is_none")]
    pub rrule: String,
    /// ID of the SLO that this correction applies to.
    #[serde(rename = "slo_id", skip_serializing_if = "Option::is_none")]
    pub slo_id: String,
    /// Starting time of the correction in epoch seconds.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: i64,
    /// The timezone to display in the UI for the correction times (defaults to "UTC").
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
}

