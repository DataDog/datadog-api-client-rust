// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes accepted when creating an Archive Search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArchiveSearchCreateRequestAttributes {
    /// ID of the archive to search. Use the Logs Archives API to list the archives of the organization.
    #[serde(rename = "archive_id")]
    pub archive_id: String,
    /// Free-text description of the Archive Search.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Start of the time range to search, as an ISO 8601 timestamp.
    #[serde(rename = "from")]
    pub from: chrono::DateTime<chrono::Utc>,
    /// Name of the Archive Search.
    #[serde(rename = "name")]
    pub name: String,
    /// Log search query used to filter the archived logs.
    #[serde(rename = "query")]
    pub query: String,
    /// Rehydration settings. Include this object to index the matched logs into a retained historical view.
    /// Omit it to run an Archive Search that only scans the archive.
    #[serde(rename = "rehydration")]
    pub rehydration: Option<crate::datadogV2::model::ArchiveSearchCreateRehydration>,
    /// End of the time range to search, as an ISO 8601 timestamp. Must be after `from`.
    #[serde(rename = "to")]
    pub to: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArchiveSearchCreateRequestAttributes {
    pub fn new(
        archive_id: String,
        from: chrono::DateTime<chrono::Utc>,
        name: String,
        query: String,
        to: chrono::DateTime<chrono::Utc>,
    ) -> ArchiveSearchCreateRequestAttributes {
        ArchiveSearchCreateRequestAttributes {
            archive_id,
            description: None,
            from,
            name,
            query,
            rehydration: None,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn rehydration(
        mut self,
        value: crate::datadogV2::model::ArchiveSearchCreateRehydration,
    ) -> Self {
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

impl<'de> Deserialize<'de> for ArchiveSearchCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArchiveSearchCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ArchiveSearchCreateRequestAttributesVisitor {
            type Value = ArchiveSearchCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archive_id: Option<String> = None;
                let mut description: Option<String> = None;
                let mut from: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut rehydration: Option<
                    crate::datadogV2::model::ArchiveSearchCreateRehydration,
                > = None;
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
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
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
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = ArchiveSearchCreateRequestAttributes {
                    archive_id,
                    description,
                    from,
                    name,
                    query,
                    rehydration,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArchiveSearchCreateRequestAttributesVisitor)
    }
}
