// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Specifies how the rotation repeats: number of days, plus optional seconds, up to the given maximums.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleUpdateRequestDataAttributesLayersItemsInterval {
    /// How many days each rotation cycle should span.
    #[serde(rename = "days")]
    pub days: Option<i32>,
    /// Additional seconds to add to the rotation cycle (for example, partial days).
    #[serde(rename = "seconds")]
    pub seconds: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleUpdateRequestDataAttributesLayersItemsInterval {
    pub fn new() -> ScheduleUpdateRequestDataAttributesLayersItemsInterval {
        ScheduleUpdateRequestDataAttributesLayersItemsInterval {
            days: None,
            seconds: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn days(mut self, value: i32) -> Self {
        self.days = Some(value);
        self
    }

    pub fn seconds(mut self, value: i64) -> Self {
        self.seconds = Some(value);
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

impl Default for ScheduleUpdateRequestDataAttributesLayersItemsInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScheduleUpdateRequestDataAttributesLayersItemsInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleUpdateRequestDataAttributesLayersItemsIntervalVisitor;
        impl<'a> Visitor<'a> for ScheduleUpdateRequestDataAttributesLayersItemsIntervalVisitor {
            type Value = ScheduleUpdateRequestDataAttributesLayersItemsInterval;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut days: Option<i32> = None;
                let mut seconds: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "days" => {
                            if v.is_null() {
                                continue;
                            }
                            days = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            seconds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScheduleUpdateRequestDataAttributesLayersItemsInterval {
                    days,
                    seconds,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScheduleUpdateRequestDataAttributesLayersItemsIntervalVisitor)
    }
}
