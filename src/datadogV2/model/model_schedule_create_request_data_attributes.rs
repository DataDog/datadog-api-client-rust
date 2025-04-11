// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Describes the main attributes for creating a new schedule, including name, layers, time zone, and tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleCreateRequestDataAttributes {
    /// The layers of on-call coverage that define rotation intervals and restrictions.
    #[serde(rename = "layers")]
    pub layers: Vec<crate::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItems>,
    /// A human-readable name for the new schedule.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of tags for categorizing or filtering the schedule.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The time zone in which the schedule is defined.
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleCreateRequestDataAttributes {
    pub fn new(
        layers: Vec<crate::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItems>,
        name: String,
        time_zone: String,
    ) -> ScheduleCreateRequestDataAttributes {
        ScheduleCreateRequestDataAttributes {
            layers,
            name,
            tags: None,
            time_zone,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for ScheduleCreateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleCreateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ScheduleCreateRequestDataAttributesVisitor {
            type Value = ScheduleCreateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut layers: Option<
                    Vec<crate::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItems>,
                > = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut time_zone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "layers" => {
                            layers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_zone" => {
                            time_zone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let layers = layers.ok_or_else(|| M::Error::missing_field("layers"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let time_zone = time_zone.ok_or_else(|| M::Error::missing_field("time_zone"))?;

                let content = ScheduleCreateRequestDataAttributes {
                    layers,
                    name,
                    tags,
                    time_zone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScheduleCreateRequestDataAttributesVisitor)
    }
}
