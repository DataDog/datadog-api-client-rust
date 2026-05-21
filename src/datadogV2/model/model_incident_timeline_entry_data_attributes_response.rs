// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a timeline entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTimelineEntryDataAttributesResponse {
    /// The type of a timeline cell.
    #[serde(rename = "cell_type")]
    pub cell_type: crate::datadogV2::model::IncidentTimelineCellType,
    /// The content of a timeline entry.
    #[serde(rename = "content")]
    pub content: crate::datadogV2::model::IncidentTimelineEntryContent,
    /// Timestamp when the entry was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The display time for this timeline entry.
    #[serde(rename = "display_time")]
    pub display_time: chrono::DateTime<chrono::Utc>,
    /// Whether this timeline entry is marked as important.
    #[serde(rename = "important")]
    pub important: bool,
    /// The incident identifier.
    #[serde(rename = "incident_id")]
    pub incident_id: String,
    /// Timestamp when the entry was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// UUID of the parent timeline entry.
    #[serde(
        rename = "parent_uuid",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parent_uuid: Option<Option<String>>,
    /// The source of a timeline cell.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::IncidentTimelineCellSource,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTimelineEntryDataAttributesResponse {
    pub fn new(
        cell_type: crate::datadogV2::model::IncidentTimelineCellType,
        content: crate::datadogV2::model::IncidentTimelineEntryContent,
        created: chrono::DateTime<chrono::Utc>,
        display_time: chrono::DateTime<chrono::Utc>,
        important: bool,
        incident_id: String,
        modified: chrono::DateTime<chrono::Utc>,
        source: crate::datadogV2::model::IncidentTimelineCellSource,
    ) -> IncidentTimelineEntryDataAttributesResponse {
        IncidentTimelineEntryDataAttributesResponse {
            cell_type,
            content,
            created,
            display_time,
            important,
            incident_id,
            modified,
            parent_uuid: None,
            source,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn parent_uuid(mut self, value: Option<String>) -> Self {
        self.parent_uuid = Some(value);
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

impl<'de> Deserialize<'de> for IncidentTimelineEntryDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTimelineEntryDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentTimelineEntryDataAttributesResponseVisitor {
            type Value = IncidentTimelineEntryDataAttributesResponse;

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
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut display_time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut important: Option<bool> = None;
                let mut incident_id: Option<String> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut parent_uuid: Option<Option<String>> = None;
                let mut source: Option<crate::datadogV2::model::IncidentTimelineCellSource> = None;
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
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_time" => {
                            display_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "important" => {
                            important = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_uuid" => {
                            parent_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::IncidentTimelineCellSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let display_time =
                    display_time.ok_or_else(|| M::Error::missing_field("display_time"))?;
                let important = important.ok_or_else(|| M::Error::missing_field("important"))?;
                let incident_id =
                    incident_id.ok_or_else(|| M::Error::missing_field("incident_id"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = IncidentTimelineEntryDataAttributesResponse {
                    cell_type,
                    content,
                    created,
                    display_time,
                    important,
                    incident_id,
                    modified,
                    parent_uuid,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTimelineEntryDataAttributesResponseVisitor)
    }
}
