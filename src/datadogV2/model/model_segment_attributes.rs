// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a segment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentAttributes {
    /// Time the segment was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A Datadog user who created, modified, or disabled a segment.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::SegmentUser>,
    /// Defines who belongs to the segment. It contains one or more blocks, up to 20 in total.
    /// Use `combination` to combine named blocks with a boolean formula such as `A AND NOT B`.
    #[serde(rename = "data_query")]
    pub data_query: crate::datadogV2::model::SegmentDataQuery,
    /// Description of the segment.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Time the segment was disabled, if it was.
    #[serde(rename = "disabled_at")]
    pub disabled_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A Datadog user who created, modified, or disabled a segment.
    #[serde(rename = "disabled_by")]
    pub disabled_by: Option<crate::datadogV2::model::SegmentUser>,
    /// Kind of entity that the segment contains, `user` or `account`.
    #[serde(rename = "entity")]
    pub entity: Option<String>,
    /// Time the segment membership was last computed.
    #[serde(
        rename = "materialized_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub materialized_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time the segment was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// A Datadog user who created, modified, or disabled a segment.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<crate::datadogV2::model::SegmentUser>,
    /// Name of the segment.
    #[serde(rename = "name")]
    pub name: String,
    /// ID of the organization that owns the segment.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// Number of entities in the segment when its membership was last computed. `-1` means the count is not
    /// available yet.
    #[serde(rename = "row_count")]
    pub row_count: Option<i64>,
    /// Who created the segment. `0` for a user and `1` for Datadog.
    #[serde(rename = "source")]
    pub source: Option<i64>,
    /// Tags of the segment.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// Kind of segment, derived from its blocks, such as `event_platform`, `user_store`, or `combination`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Version of the segment, increased on each update.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        data_query: crate::datadogV2::model::SegmentDataQuery,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
    ) -> SegmentAttributes {
        SegmentAttributes {
            created_at,
            created_by: None,
            data_query,
            description: None,
            disabled_at: None,
            disabled_by: None,
            entity: None,
            materialized_at: None,
            modified_at,
            modified_by: None,
            name,
            org_id: None,
            row_count: None,
            source: None,
            tags: None,
            type_: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(mut self, value: crate::datadogV2::model::SegmentUser) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn disabled_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.disabled_at = Some(value);
        self
    }

    pub fn disabled_by(mut self, value: crate::datadogV2::model::SegmentUser) -> Self {
        self.disabled_by = Some(value);
        self
    }

    pub fn entity(mut self, value: String) -> Self {
        self.entity = Some(value);
        self
    }

    pub fn materialized_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.materialized_at = Some(value);
        self
    }

    pub fn modified_by(mut self, value: crate::datadogV2::model::SegmentUser) -> Self {
        self.modified_by = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn source(mut self, value: i64) -> Self {
        self.source = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for SegmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentAttributesVisitor {
            type Value = SegmentAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::SegmentUser> = None;
                let mut data_query: Option<crate::datadogV2::model::SegmentDataQuery> = None;
                let mut description: Option<String> = None;
                let mut disabled_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut disabled_by: Option<crate::datadogV2::model::SegmentUser> = None;
                let mut entity: Option<String> = None;
                let mut materialized_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<crate::datadogV2::model::SegmentUser> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut row_count: Option<i64> = None;
                let mut source: Option<i64> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut type_: Option<String> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_query" => {
                            data_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_at" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity" => {
                            if v.is_null() {
                                continue;
                            }
                            entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "materialized_at" => {
                            materialized_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "row_count" => {
                            if v.is_null() {
                                continue;
                            }
                            row_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let data_query = data_query.ok_or_else(|| M::Error::missing_field("data_query"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = SegmentAttributes {
                    created_at,
                    created_by,
                    data_query,
                    description,
                    disabled_at,
                    disabled_by,
                    entity,
                    materialized_at,
                    modified_at,
                    modified_by,
                    name,
                    org_id,
                    row_count,
                    source,
                    tags,
                    type_,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentAttributesVisitor)
    }
}
