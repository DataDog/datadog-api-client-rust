// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a timeline entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTimelineEntryDataAttributesRequest {
    /// The type of a timeline cell.
    #[serde(rename = "cell_type")]
    pub cell_type: crate::datadogV2::model::IncidentTimelineCellType,
    /// The content of a timeline entry.
    #[serde(rename = "content")]
    pub content: crate::datadogV2::model::IncidentTimelineEntryContent,
    /// The display time for this timeline entry.
    #[serde(rename = "display_time")]
    pub display_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether this timeline entry is marked as important.
    #[serde(rename = "important")]
    pub important: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTimelineEntryDataAttributesRequest {
    pub fn new(
        cell_type: crate::datadogV2::model::IncidentTimelineCellType,
        content: crate::datadogV2::model::IncidentTimelineEntryContent,
    ) -> IncidentTimelineEntryDataAttributesRequest {
        IncidentTimelineEntryDataAttributesRequest {
            cell_type,
            content,
            display_time: None,
            important: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_time(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.display_time = Some(value);
        self
    }

    pub fn important(mut self, value: bool) -> Self {
        self.important = Some(value);
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

impl<'de> Deserialize<'de> for IncidentTimelineEntryDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTimelineEntryDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentTimelineEntryDataAttributesRequestVisitor {
            type Value = IncidentTimelineEntryDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cell_type: Option<crate::datadogV2::model::IncidentTimelineCellType> = None;
                let mut content: Option<crate::datadogV2::model::IncidentTimelineEntryContent> =
                    None;
                let mut display_time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut important: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cell_type" => {
                            cell_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_type) = cell_type {
                                match _cell_type {
                                    crate::datadogV2::model::IncidentTimelineCellType::UnparsedObject(_cell_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_time" => {
                            if v.is_null() {
                                continue;
                            }
                            display_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "important" => {
                            if v.is_null() {
                                continue;
                            }
                            important = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cell_type = cell_type.ok_or_else(|| M::Error::missing_field("cell_type"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;

                let content = IncidentTimelineEntryDataAttributesRequest {
                    cell_type,
                    content,
                    display_time,
                    important,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTimelineEntryDataAttributesRequestVisitor)
    }
}
