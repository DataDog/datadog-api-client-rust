// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a segment in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSegmentResponseAttributes {
    /// The creation timestamp in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A user who performed an action on a segment.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::RumSegmentUser,
    /// Query definition for the segment. Contains one or more query blocks and an optional combination formula.
    #[serde(rename = "data_query")]
    pub data_query: crate::datadogV2::model::RumSegmentDataQuery,
    /// A description of the segment.
    #[serde(rename = "description")]
    pub description: String,
    /// The last modification timestamp in RFC 3339 format.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// A user who performed an action on a segment.
    #[serde(rename = "modified_by")]
    pub modified_by: crate::datadogV2::model::RumSegmentUser,
    /// The name of the segment.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization identifier.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The number of users in the segment.
    #[serde(rename = "row_count")]
    pub row_count: i64,
    /// The source of a segment.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::RumSegmentSource,
    /// A list of tags for the segment.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The type of a segment based on its data query configuration.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RumSegmentSegmentType,
    /// The unique identifier of the segment.
    #[serde(rename = "uuid")]
    pub uuid: String,
    /// The version number of the segment.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSegmentResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: crate::datadogV2::model::RumSegmentUser,
        data_query: crate::datadogV2::model::RumSegmentDataQuery,
        description: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: crate::datadogV2::model::RumSegmentUser,
        name: String,
        org_id: i64,
        row_count: i64,
        source: crate::datadogV2::model::RumSegmentSource,
        tags: Vec<String>,
        type_: crate::datadogV2::model::RumSegmentSegmentType,
        uuid: String,
        version: i64,
    ) -> RumSegmentResponseAttributes {
        RumSegmentResponseAttributes {
            created_at,
            created_by,
            data_query,
            description,
            modified_at,
            modified_by,
            name,
            org_id,
            row_count,
            source,
            tags,
            type_,
            uuid,
            version,
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

impl<'de> Deserialize<'de> for RumSegmentResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSegmentResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RumSegmentResponseAttributesVisitor {
            type Value = RumSegmentResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::RumSegmentUser> = None;
                let mut data_query: Option<crate::datadogV2::model::RumSegmentDataQuery> = None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<crate::datadogV2::model::RumSegmentUser> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut row_count: Option<i64> = None;
                let mut source: Option<crate::datadogV2::model::RumSegmentSource> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::RumSegmentSegmentType> = None;
                let mut uuid: Option<String> = None;
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
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_query" => {
                            data_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "row_count" => {
                            row_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::RumSegmentSource::UnparsedObject(
                                        _source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RumSegmentSegmentType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
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
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let data_query = data_query.ok_or_else(|| M::Error::missing_field("data_query"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let row_count = row_count.ok_or_else(|| M::Error::missing_field("row_count"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = RumSegmentResponseAttributes {
                    created_at,
                    created_by,
                    data_query,
                    description,
                    modified_at,
                    modified_by,
                    name,
                    org_id,
                    row_count,
                    source,
                    tags,
                    type_,
                    uuid,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSegmentResponseAttributesVisitor)
    }
}
