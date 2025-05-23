// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the updatable attributes for a schedule, such as name, time zone, and layers.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleUpdateRequestDataAttributes {
    /// The updated list of layers (rotations) for this schedule.
    #[serde(rename = "layers")]
    pub layers: Vec<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItems>,
    /// A short name for the schedule.
    #[serde(rename = "name")]
    pub name: String,
    /// The time zone used when interpreting rotation times.
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleUpdateRequestDataAttributes {
    pub fn new(
        layers: Vec<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItems>,
        name: String,
        time_zone: String,
    ) -> ScheduleUpdateRequestDataAttributes {
        ScheduleUpdateRequestDataAttributes {
            layers,
            name,
            time_zone,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ScheduleUpdateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleUpdateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ScheduleUpdateRequestDataAttributesVisitor {
            type Value = ScheduleUpdateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut layers: Option<
                    Vec<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItems>,
                > = None;
                let mut name: Option<String> = None;
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

                let content = ScheduleUpdateRequestDataAttributes {
                    layers,
                    name,
                    time_zone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScheduleUpdateRequestDataAttributesVisitor)
    }
}
