// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionResponseAttributes {
    /// Category the SLO correction belongs to.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: SLOCorrectionCategory,
    /// The epoch timestamp of when the correction was created at.
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub created_at: Option<Int64>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Creator,
    /// Description of the correction being made.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Length of time (in seconds) for a specified `rrule` recurring SLO correction.
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub duration: Option<Int64>,
    /// Ending time of the correction in epoch seconds.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub end: Option<Int64>,
    /// The epoch timestamp of when the correction was modified at.
    #[serde(rename = "modified_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub modified_at: Option<Int64>,
    /// Modifier of the object.
    #[serde(rename = "modifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub modifier: NullableSLOCorrectionResponseAttributesModifier,
    /// The recurrence rules as defined in the iCalendar RFC 5545. The supported rules for SLO corrections
are `FREQ`, `INTERVAL`, `COUNT`, and `UNTIL`.
    #[serde(rename = "rrule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub rrule: Option<String>,
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

