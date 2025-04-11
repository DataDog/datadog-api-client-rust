// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a time restriction object for a layer within a schedule update, including
/// start and end days, as well as times.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
    /// Defines the day of the week on which the time restriction ends.
    #[serde(rename = "end_day")]
    pub end_day: Option<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay>,
    /// The time at which this restriction ends (hh:mm:ss).
    #[serde(rename = "end_time")]
    pub end_time: Option<String>,
    /// Defines the day of the week on which the time restriction starts.
    #[serde(rename = "start_day")]
    pub start_day: Option<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsStartDay>,
    /// The time at which this restriction starts (hh:mm:ss).
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
    pub fn new() -> ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
        ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
            end_day: None,
            end_time: None,
            start_day: None,
            start_time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end_day(
        mut self,
        value: crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay,
    ) -> Self {
        self.end_day = Some(value);
        self
    }

    pub fn end_time(mut self, value: String) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn start_day(
        mut self,
        value: crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsStartDay,
    ) -> Self {
        self.start_day = Some(value);
        self
    }

    pub fn start_time(mut self, value: String) -> Self {
        self.start_time = Some(value);
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

impl Default for ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsVisitor;
        impl<'a> Visitor<'a> for ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsVisitor {
            type Value = ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end_day: Option<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay> = None;
                let mut end_time: Option<String> = None;
                let mut start_day: Option<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsStartDay> = None;
                let mut start_time: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end_day" => {
                            if v.is_null() {
                                continue;
                            }
                            end_day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _end_day) = end_day {
                                match _end_day {
                                    crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay::UnparsedObject(_end_day) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "end_time" => {
                            if v.is_null() {
                                continue;
                            }
                            end_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_day" => {
                            if v.is_null() {
                                continue;
                            }
                            start_day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _start_day) = start_day {
                                match _start_day {
                                    crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsStartDay::UnparsedObject(_start_day) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems {
                    end_day,
                    end_time,
                    start_day,
                    start_time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsVisitor)
    }
}
