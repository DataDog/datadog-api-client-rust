// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attribute object associated with the SLO correction to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCorrectionCreateRequestAttributes {
    /// Category the SLO correction belongs to.
    #[serde(rename = "category")]
    pub category: crate::datadogV1::model::SLOCorrectionCategory,
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
    /// are `FREQ`, `INTERVAL`, `COUNT` and `UNTIL`.
    #[serde(rename = "rrule")]
    pub rrule: Option<String>,
    /// ID of the SLO that this correction applies to.
    #[serde(rename = "slo_id")]
    pub slo_id: String,
    /// Starting time of the correction in epoch seconds.
    #[serde(rename = "start")]
    pub start: i64,
    /// The timezone to display in the UI for the correction times (defaults to "UTC").
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCorrectionCreateRequestAttributes {
    pub fn new(
        category: crate::datadogV1::model::SLOCorrectionCategory,
        slo_id: String,
        start: i64,
    ) -> SLOCorrectionCreateRequestAttributes {
        SLOCorrectionCreateRequestAttributes {
            category,
            description: None,
            duration: None,
            end: None,
            rrule: None,
            slo_id,
            start,
            timezone: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn rrule(mut self, value: String) -> Self {
        self.rrule = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SLOCorrectionCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCorrectionCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SLOCorrectionCreateRequestAttributesVisitor {
            type Value = SLOCorrectionCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<crate::datadogV1::model::SLOCorrectionCategory> = None;
                let mut description: Option<String> = None;
                let mut duration: Option<i64> = None;
                let mut end: Option<i64> = None;
                let mut rrule: Option<String> = None;
                let mut slo_id: Option<String> = None;
                let mut start: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
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
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            if v.is_null() {
                                continue;
                            }
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_id" => {
                            slo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
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
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let slo_id = slo_id.ok_or_else(|| M::Error::missing_field("slo_id"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = SLOCorrectionCreateRequestAttributes {
                    category,
                    description,
                    duration,
                    end,
                    rrule,
                    slo_id,
                    start,
                    timezone,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCorrectionCreateRequestAttributesVisitor)
    }
}
