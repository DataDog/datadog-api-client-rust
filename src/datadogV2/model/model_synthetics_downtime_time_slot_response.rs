// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A time slot returned in a Synthetics downtime response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDowntimeTimeSlotResponse {
    /// The duration of the time slot in seconds.
    #[serde(rename = "duration")]
    pub duration: i64,
    /// The unique identifier of the time slot.
    #[serde(rename = "id")]
    pub id: String,
    /// The label for the time slot.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Recurrence settings returned in a Synthetics downtime time slot response.
    #[serde(rename = "recurrence")]
    pub recurrence: Option<crate::datadogV2::model::SyntheticsDowntimeTimeSlotRecurrenceResponse>,
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

impl SyntheticsDowntimeTimeSlotResponse {
    pub fn new(
        duration: i64,
        id: String,
        start: crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate,
        timezone: String,
    ) -> SyntheticsDowntimeTimeSlotResponse {
        SyntheticsDowntimeTimeSlotResponse {
            duration,
            id,
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
        value: crate::datadogV2::model::SyntheticsDowntimeTimeSlotRecurrenceResponse,
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

impl<'de> Deserialize<'de> for SyntheticsDowntimeTimeSlotResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDowntimeTimeSlotResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsDowntimeTimeSlotResponseVisitor {
            type Value = SyntheticsDowntimeTimeSlotResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut recurrence: Option<
                    crate::datadogV2::model::SyntheticsDowntimeTimeSlotRecurrenceResponse,
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
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;

                let content = SyntheticsDowntimeTimeSlotResponse {
                    duration,
                    id,
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

        deserializer.deserialize_any(SyntheticsDowntimeTimeSlotResponseVisitor)
    }
}
