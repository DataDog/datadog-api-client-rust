// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attribute object associated with the SLO correction.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub creator: Option<crate::datadogV1::model::Creator>,
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
    pub modifier: Option<Option<crate::datadogV1::model::SLOCorrectionResponseAttributesModifier>>,
    /// The recurrence rules as defined in the iCalendar RFC 5545. The supported rules for SLO corrections
    /// are `FREQ`, `INTERVAL`, `COUNT`, `UNTIL` and `BYDAY`.
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: crate::datadogV1::model::SLOCorrectionCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn created_at(mut self, value: Option<i64>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn duration(mut self, value: Option<i64>) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn end(mut self, value: Option<i64>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn modified_at(mut self, value: Option<i64>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn modifier(
        mut self,
        value: Option<crate::datadogV1::model::SLOCorrectionResponseAttributesModifier>,
    ) -> Self {
        self.modifier = Some(value);
        self
    }

    pub fn rrule(mut self, value: Option<String>) -> Self {
        self.rrule = Some(value);
        self
    }

    pub fn slo_id(mut self, value: String) -> Self {
        self.slo_id = Some(value);
        self
    }

    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl Default for SLOCorrectionResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOCorrectionResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCorrectionResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SLOCorrectionResponseAttributesVisitor {
            type Value = SLOCorrectionResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<crate::datadogV1::model::SLOCorrectionCategory> = None;
                let mut created_at: Option<Option<i64>> = None;
                let mut creator: Option<crate::datadogV1::model::Creator> = None;
                let mut description: Option<String> = None;
                let mut duration: Option<Option<i64>> = None;
                let mut end: Option<Option<i64>> = None;
                let mut modified_at: Option<Option<i64>> = None;
                let mut modifier: Option<
                    Option<crate::datadogV1::model::SLOCorrectionResponseAttributesModifier>,
                > = None;
                let mut rrule: Option<Option<String>> = None;
                let mut slo_id: Option<String> = None;
                let mut start: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV1::model::SLOCorrectionCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modifier" => {
                            modifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_id" => {
                            if v.is_null() {
                                continue;
                            }
                            slo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SLOCorrectionResponseAttributes {
                    category,
                    created_at,
                    creator,
                    description,
                    duration,
                    end,
                    modified_at,
                    modifier,
                    rrule,
                    slo_id,
                    start,
                    timezone,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCorrectionResponseAttributesVisitor)
    }
}
