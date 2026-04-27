// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A time slot for a Synthetics downtime create or update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDowntimeTimeSlotRequest {
    /// The duration of the time slot in seconds, between 60 and 604800.
    #[serde(rename = "duration")]
    pub duration: i64,
    /// An optional label for the time slot.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Recurrence settings for a Synthetics downtime time slot.
    #[serde(rename = "recurrence")]
    pub recurrence: Option<crate::datadogV2::model::SyntheticsDowntimeTimeSlotRecurrenceRequest>,
    /// A specific date and time used to define the start or end of a Synthetics downtime time slot.
    #[serde(rename = "start")]
    pub start: crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate,
    /// The IANA timezone name for the time slot.
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDowntimeTimeSlotRequest {
    pub fn new(
        duration: i64,
        start: crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate,
        timezone: String,
    ) -> SyntheticsDowntimeTimeSlotRequest {
        SyntheticsDowntimeTimeSlotRequest {
            duration,
            name: None,
            recurrence: None,
            start,
            timezone,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn recurrence(
        mut self,
        value: crate::datadogV2::model::SyntheticsDowntimeTimeSlotRecurrenceRequest,
    ) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsDowntimeTimeSlotRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDowntimeTimeSlotRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsDowntimeTimeSlotRequestVisitor {
            type Value = SyntheticsDowntimeTimeSlotRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut recurrence: Option<
                    crate::datadogV2::model::SyntheticsDowntimeTimeSlotRecurrenceRequest,
                > = None;
                let mut start: Option<crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate> =
                    None;
                let mut timezone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recurrence" => {
                            if v.is_null() {
                                continue;
                            }
                            recurrence = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;

                let content = SyntheticsDowntimeTimeSlotRequest {
                    duration,
                    name,
                    recurrence,
                    start,
                    timezone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDowntimeTimeSlotRequestVisitor)
    }
}
