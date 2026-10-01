// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an Archive Search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArchiveSearchResponseAttributes {
    /// ID of the archive being searched.
    #[serde(rename = "archive_id")]
    pub archive_id: String,
    /// Number of bytes read from the archive at the end of the search.
    #[serde(rename = "bytes_scanned")]
    pub bytes_scanned: i64,
    /// Time the Archive Search finished, as an ISO 8601 timestamp.
    /// Absent while the search is still running.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Time the Archive Search was created, as an ISO 8601 timestamp.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Free-text description of the Archive Search.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Number of events read from the archive at the end of the search.
    #[serde(rename = "events_scanned")]
    pub events_scanned: i64,
    /// Estimated time left before the Archive Search completes, in seconds.
    #[serde(rename = "expected_duration")]
    pub expected_duration: Option<i64>,
    /// Start of the searched time range, as an ISO 8601 timestamp.
    #[serde(rename = "from")]
    pub from: chrono::DateTime<chrono::Utc>,
    /// Name of the Archive Search.
    #[serde(rename = "name")]
    pub name: String,
    /// Log search query used to filter the archived logs.
    #[serde(rename = "query")]
    pub query: String,
    /// Rehydration settings of the Archive Search. Absent when the search only scans the archive
    /// without indexing the results.
    #[serde(rename = "rehydration")]
    pub rehydration: Option<crate::datadogV2::model::ArchiveSearchRehydration>,
    /// Current state of an Archive Search.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ArchiveSearchStatus,
    /// End of the searched time range, as an ISO 8601 timestamp.
    #[serde(rename = "to")]
    pub to: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArchiveSearchResponseAttributes {
    pub fn new(
        archive_id: String,
        bytes_scanned: i64,
        created_at: chrono::DateTime<chrono::Utc>,
        events_scanned: i64,
        from: chrono::DateTime<chrono::Utc>,
        name: String,
        query: String,
        status: crate::datadogV2::model::ArchiveSearchStatus,
        to: chrono::DateTime<chrono::Utc>,
    ) -> ArchiveSearchResponseAttributes {
        ArchiveSearchResponseAttributes {
            archive_id,
            bytes_scanned,
            completed_at: None,
            created_at,
            description: None,
            events_scanned,
            expected_duration: None,
            from,
            name,
            query,
            rehydration: None,
            status,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn completed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn expected_duration(mut self, value: i64) -> Self {
        self.expected_duration = Some(value);
        self
    }

    pub fn rehydration(mut self, value: crate::datadogV2::model::ArchiveSearchRehydration) -> Self {
        self.rehydration = Some(value);
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

impl<'de> Deserialize<'de> for ArchiveSearchResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArchiveSearchResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ArchiveSearchResponseAttributesVisitor {
            type Value = ArchiveSearchResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archive_id: Option<String> = None;
                let mut bytes_scanned: Option<i64> = None;
                let mut completed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut events_scanned: Option<i64> = None;
                let mut expected_duration: Option<i64> = None;
                let mut from: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut rehydration: Option<crate::datadogV2::model::ArchiveSearchRehydration> =
                    None;
                let mut status: Option<crate::datadogV2::model::ArchiveSearchStatus> = None;
                let mut to: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archive_id" => {
                            archive_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bytes_scanned" => {
                            bytes_scanned =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "completed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            completed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events_scanned" => {
                            events_scanned =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expected_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            expected_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rehydration" => {
                            if v.is_null() {
                                continue;
                            }
                            rehydration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ArchiveSearchStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let archive_id = archive_id.ok_or_else(|| M::Error::missing_field("archive_id"))?;
                let bytes_scanned =
                    bytes_scanned.ok_or_else(|| M::Error::missing_field("bytes_scanned"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let events_scanned =
                    events_scanned.ok_or_else(|| M::Error::missing_field("events_scanned"))?;
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = ArchiveSearchResponseAttributes {
                    archive_id,
                    bytes_scanned,
                    completed_at,
                    created_at,
                    description,
                    events_scanned,
                    expected_duration,
                    from,
                    name,
                    query,
                    rehydration,
                    status,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArchiveSearchResponseAttributesVisitor)
    }
}
