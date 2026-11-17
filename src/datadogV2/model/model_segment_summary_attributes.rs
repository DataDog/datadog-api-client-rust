// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a segment in a list. The segment definition is not included.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentSummaryAttributes {
    /// Time the segment was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A Datadog user who created, modified, or disabled a segment.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::SegmentUser>,
    /// Description of the segment.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Kind of entity that the segment contains, `user` or `account`.
    #[serde(rename = "entity")]
    pub entity: Option<String>,
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
    /// Kind of segment, derived from its blocks.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentSummaryAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
    ) -> SegmentSummaryAttributes {
        SegmentSummaryAttributes {
            created_at,
            created_by: None,
            description: None,
            entity: None,
            modified_at,
            modified_by: None,
            name,
            org_id: None,
            row_count: None,
            source: None,
            tags: None,
            type_: None,
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

    pub fn entity(mut self, value: String) -> Self {
        self.entity = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SegmentSummaryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentSummaryAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentSummaryAttributesVisitor {
            type Value = SegmentSummaryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::SegmentUser> = None;
                let mut description: Option<String> = None;
                let mut entity: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<crate::datadogV2::model::SegmentUser> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut row_count: Option<i64> = None;
                let mut source: Option<i64> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut type_: Option<String> = None;
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
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity" => {
                            if v.is_null() {
                                continue;
                            }
                            entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = SegmentSummaryAttributes {
                    created_at,
                    created_by,
                    description,
                    entity,
                    modified_at,
                    modified_by,
                    name,
                    org_id,
                    row_count,
                    source,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentSummaryAttributesVisitor)
    }
}
