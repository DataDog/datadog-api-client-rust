// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Represents a layer within a schedule update, including rotation details, members,
/// and optional restrictions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleUpdateRequestDataAttributesLayersItems {
    /// When this updated layer takes effect (ISO 8601 format).
    #[serde(rename = "effective_date")]
    pub effective_date: chrono::DateTime<chrono::Utc>,
    /// When this updated layer should stop being active (ISO 8601 format).
    #[serde(rename = "end_date")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// A unique identifier for the layer being updated.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies how the rotation repeats: number of days, plus optional seconds, up to the given maximums.
    #[serde(rename = "interval")]
    pub interval: crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsInterval,
    /// The members assigned to this layer.
    #[serde(rename = "members")]
    pub members:
        Vec<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItems>,
    /// The name for this layer (for example, "Secondary Coverage").
    #[serde(rename = "name")]
    pub name: String,
    /// Any time restrictions that define when this layer is active.
    #[serde(rename = "restrictions")]
    pub restrictions: Option<Vec<crate::datadogV2::model::TimeRestriction>>,
    /// The date/time at which the rotation begins (ISO 8601 format).
    #[serde(rename = "rotation_start")]
    pub rotation_start: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleUpdateRequestDataAttributesLayersItems {
    pub fn new(
        effective_date: chrono::DateTime<chrono::Utc>,
        interval: crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsInterval,
        members: Vec<
            crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItems,
        >,
        name: String,
        rotation_start: chrono::DateTime<chrono::Utc>,
    ) -> ScheduleUpdateRequestDataAttributesLayersItems {
        ScheduleUpdateRequestDataAttributesLayersItems {
            effective_date,
            end_date: None,
            id: None,
            interval,
            members,
            name,
            restrictions: None,
            rotation_start,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn restrictions(mut self, value: Vec<crate::datadogV2::model::TimeRestriction>) -> Self {
        self.restrictions = Some(value);
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

impl<'de> Deserialize<'de> for ScheduleUpdateRequestDataAttributesLayersItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleUpdateRequestDataAttributesLayersItemsVisitor;
        impl<'a> Visitor<'a> for ScheduleUpdateRequestDataAttributesLayersItemsVisitor {
            type Value = ScheduleUpdateRequestDataAttributesLayersItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut effective_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut end_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<String> = None;
                let mut interval: Option<
                    crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsInterval,
                > = None;
                let mut members: Option<Vec<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItems>> = None;
                let mut name: Option<String> = None;
                let mut restrictions: Option<Vec<crate::datadogV2::model::TimeRestriction>> = None;
                let mut rotation_start: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "effective_date" => {
                            effective_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_date" => {
                            if v.is_null() {
                                continue;
                            }
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "members" => {
                            members = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restrictions" => {
                            if v.is_null() {
                                continue;
                            }
                            restrictions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rotation_start" => {
                            rotation_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let effective_date =
                    effective_date.ok_or_else(|| M::Error::missing_field("effective_date"))?;
                let interval = interval.ok_or_else(|| M::Error::missing_field("interval"))?;
                let members = members.ok_or_else(|| M::Error::missing_field("members"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rotation_start =
                    rotation_start.ok_or_else(|| M::Error::missing_field("rotation_start"))?;

                let content = ScheduleUpdateRequestDataAttributesLayersItems {
                    effective_date,
                    end_date,
                    id,
                    interval,
                    members,
                    name,
                    restrictions,
                    rotation_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScheduleUpdateRequestDataAttributesLayersItemsVisitor)
    }
}
